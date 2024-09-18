use std::path::{Path, PathBuf};

use crate::{
    c_println,
    compiler::{Compiler, ZigTargets},
    data::parsers::ParserInfo,
    utils::{archives, command, http},
};

#[derive(Debug, Default, Clone, clap::ValueEnum)]
pub enum NodePackageManagers {
    #[default]
    Npm,
    Pnpm,
    Yarn,
    #[cfg(unix)]
    Bun,
}

impl NodePackageManagers {
    #[cfg(unix)]
    fn get_name(&self) -> &str {
        match self {
            NodePackageManagers::Npm => "npm",
            NodePackageManagers::Pnpm => "pnpm",
            NodePackageManagers::Yarn => "yarn",
            NodePackageManagers::Bun => "bun",
        }
    }

    #[cfg(windows)]
    fn get_name(&self) -> &str {
        match self {
            NodePackageManagers::Npm => "npm.cmd",
            NodePackageManagers::Pnpm => "pnpm.cmd",
            NodePackageManagers::Yarn => "yarn.cmd",
        }
    }
}

pub fn check_compile_deps(
    compiler: &dyn Compiler,
    npm: &NodePackageManagers,
) -> anyhow::Result<()> {
    command::check_exists(compiler.get_name())?;
    command::check_exists(npm.get_name())?;
    command::check_exists("tree-sitter")?;
    Ok(())
}

pub fn uninstall(lang: &str, destination: &Path) -> anyhow::Result<()> {
    let lang_so = format!("{lang}.so");
    let lang_so = PathBuf::from(lang_so);
    let lang_so = destination.join(lang_so);
    if lang_so.exists() {
        std::fs::remove_file(lang_so)?;
    } else {
        anyhow::bail!("Parser {lang} is not installed");
    }
    Ok(())
}

pub async fn compile(
    lang: &str,
    parser_info: &ParserInfo,
    compiler: &dyn Compiler,
    target: &Option<ZigTargets>,
    npm: &NodePackageManagers,
    from_grammar: bool,
    destination: &Path,
) -> anyhow::Result<()> {
    let tmp_dir = tempfile::tempdir()?;
    let download_location = tmp_dir.path().join(format!("{lang}.tar.gz"));
    let download_url = download_url(parser_info);
    let extract_dir = tmp_dir.path().join(lang);
    let mut cwd = PathBuf::new();

    http::download_file(&download_url, &download_location).await?;
    archives::extract_tar_gz(&download_location, &extract_dir)?;

    for entry in std::fs::read_dir(&extract_dir)? {
        let entry = entry?;
        cwd = entry.path();
    }

    if let Some(location) = &parser_info.location {
        cwd = cwd.join(location);
    }

    if from_grammar || parser_info.generate_from_grammar {
        if let Err(e) = generate_from_grammar(npm, &cwd).await {
            c_println!(amber, "=> WARNGING: tree-sitter generate failed: {}", e);
        }
    }

    let files = parser_info
        .files
        .iter()
        .map(|f| f.as_str())
        .collect::<Vec<_>>();
    compiler.compile(&files, &cwd, target).await?;

    let destination = PathBuf::from(shellexpand::full(destination.to_str().unwrap())?.to_string());
    std::fs::create_dir_all(&destination)?;
    std::fs::copy(cwd.join("out.so"), destination.join(format!("{lang}.so")))?;
    Ok(())
}

fn download_url(parser_info: &ParserInfo) -> String {
    let url = parser_info.url.trim_end_matches(".git");
    let repo_name = url.split('/').last().unwrap();

    match parser_info.url.contains("gitlab") {
        true => format!(
            "{url}/-/archive/{}/{repo_name}-{}.tar.gz",
            parser_info.revision, parser_info.revision
        ),
        false => format!("{url}/archive/{}.tar.gz", parser_info.revision),
    }
}

async fn generate_from_grammar(npm: &NodePackageManagers, cwd: &Path) -> anyhow::Result<()> {
    command::run(npm.get_name(), &["install"], Some(&cwd)).await?;
    command::run("tree-sitter", &["generate"], Some(&cwd)).await?;
    Ok(())
}
