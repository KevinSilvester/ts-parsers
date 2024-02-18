#![allow(clippy::borrowed_box)]

use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::{
    c_println,
    compiler::{Compiler, ZigTargets},
    data::{parsers::ParserInfo, state::State},
    utils::{command::run_command, http::download_file},
};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    // pub async fn install(
    //     parser_info: &ParserInfo,
    //     compiler: &Box<dyn Compiler + Send + Sync>,
    //     target: &Option<ZigTargets>,
    //     state: &State,
    // ) -> anyhow::Result<()> {
    //     Ok(())
    // }

    // pub async fn uninstall(parser_info: &ParserInfo, state: &State) -> anyhow::Result<()> {
    //     Ok(())
    // }
    // pub async fn update(
    //     parser_info: &ParserInfo,
    //     compiler: &Box<dyn Compiler + Send + Sync>,
    //     target: &Option<ZigTargets>,
    //     state: &State,
    // ) -> anyhow::Result<()> {
    //     Ok(())
    // }

    pub async fn compile(
        lang: &str,
        parser_info: &ParserInfo,
        compiler: &Box<dyn Compiler>,
        target: &Option<ZigTargets>,
        destination: &Path,
    ) -> anyhow::Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let download_location = tmp_dir.path().join(format!("{lang}.zip"));

        let download_url = match parser_info.url.contains("gitlab") {
            true => {
                let url = parser_info.url.trim_end_matches(".git");
                let repo_name = url.split('/').last().unwrap();
                format!(
                    "{url}/-/archive/{}/{repo_name}-{}.zip",
                    parser_info.revision, parser_info.revision
                )
            }
            false => format!("{}/archive/{}.zip", parser_info.url, parser_info.revision),
        };
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
            run_command("pnpm", &["install"], Some(&cwd)).await?;
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
