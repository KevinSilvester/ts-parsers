#![allow(clippy::borrowed_box)]

use std::path::{Path, PathBuf};

use zip::ZipArchive;

#[cfg(unix)]
const PNPM: &str = "pnpm";

#[cfg(windows)]
const PNPM: &str = "pnpm.cmd";

use crate::{
    c_println,
    compiler::{Compiler, ZigTargets},
    data::parsers::ParserInfo,
    utils::{
        command::{check_command_exists, run_command},
        http::download_file,
    },
};

fn download_url(parser_info: &ParserInfo) -> String {
    let url = parser_info.url.trim_end_matches(".git");
    let repo_name = url.split('/').last().unwrap();

    match parser_info.url.contains("gitlab") {
        true => format!(
            "{url}/-/archive/{}/{repo_name}-{}.zip",
            parser_info.revision, parser_info.revision
        ),
        false => format!("{url}/archive/{}.zip", parser_info.revision),
    }
}

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn check_compile_deps(compiler: &Box<dyn Compiler>) -> anyhow::Result<()> {
        check_command_exists(compiler.get_name())?;
        check_command_exists(PNPM)?;
        check_command_exists("tree-sitter")?;
        Ok(())
    }

    pub async fn compile(
        lang: &str,
        parser_info: &ParserInfo,
        compiler: &Box<dyn Compiler>,
        target: &Option<ZigTargets>,
        destination: &Path,
    ) -> anyhow::Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let download_location = tmp_dir.path().join(format!("{lang}.zip"));

        let download_url = download_url(parser_info);
        download_file(&download_url, &download_location).await?;

        let extract_dir = tmp_dir.path().join(lang);
        let zip_file = std::fs::File::open(&download_location)?;
        let mut zip = ZipArchive::new(zip_file)?;
        zip.extract(&extract_dir)?;

        let mut cwd = PathBuf::new();

        for entry in std::fs::read_dir(&extract_dir)? {
            let entry = entry?;
            cwd = entry.path();
        }

        if let Some(location) = parser_info.location.as_ref() {
            cwd = cwd.join(location);
        }

        if parser_info.generate_from_grammar {
            run_command(PNPM, &["install"], Some(&cwd)).await?;
            if !run_command("tree-sitter", &["generate"], Some(&cwd)).await? {
                c_println!(amber, "=> WARNGING: tree-sitter generate failed");
            }
        }

        let files = parser_info
            .files
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>();
        compiler.compile(&files, &cwd, target).await?;

        let destination =
            PathBuf::from(shellexpand::full(destination.to_str().unwrap())?.to_string());
        std::fs::create_dir_all(&destination)?;
        std::fs::copy(cwd.join("out.so"), destination.join(format!("{lang}.so")))?;
        Ok(())
    }
}
