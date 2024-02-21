use crate::{
    c_println,
    compiler::{Clang, Compiler, Zig, ZigTargets},
    data::{
        changelog::ChangeLog,
        parsers::Parsers,
        state::{ParserInstallMethod, State},
    },
    parser::Parser,
    utils::{
        command::check_command_exists,
        fs::{copy_all, remove_all},
        Backups, PATHS,
    },
};

use super::Subcommand;

#[derive(Debug, Default, Clone, clap::ValueEnum)]
enum Compilers {
    #[default]
    Clang,
    Zig,
}

#[derive(Debug, clap::Args)]
pub struct Install {
    /// Installation method
    #[clap(short, long, default_value_t, value_enum)]
    method: ParserInstallMethod,

    /// Compiler to use
    #[clap(short, long, default_value_t, value_enum)]
    compiler: Compilers,

    /// Zig target, only used when compiler is zig
    #[clap(short, long, value_enum)]
    target: Option<ZigTargets>,

    /// Compile all parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// `nvim-treesitter-parsers` release to use
    #[clap(long)]
    tag: Option<String>,

    /// Compile parsers in `wanted` list
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"] )]
    wanted: bool,

    /// Parsers to compile (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,

    /// Force install, even if parsers are already installed
    #[clap(long, default_value = "false")]
    force: bool,
}

impl Install {
    fn select_compiler(&self) -> Box<dyn Compiler> {
        match self.compiler {
            Compilers::Clang => Box::new(Clang::new()),
            Compilers::Zig => Box::new(Zig::new()),
        }
    }

    fn select_tag(&self, changelog: &ChangeLog) -> String {
        match &self.tag {
            Some(tag) => tag.clone(),
            None => changelog.get_latest_tag().unwrap(),
        }
    }

    fn select_langs(&self, parsers: &Parsers) -> anyhow::Result<Vec<String>> {
        if self.all {
            return Ok(parsers.langs.clone());
        }

        let langs = match self.wanted {
            true => {
                if parsers.wanted.is_none() {
                    return Err(anyhow::anyhow!("No wanted parsers found"));
                }
                parsers.wanted.clone().unwrap()
            }
            false => self.parsers.clone(),
        };

        parsers.validate_parsers(&langs)?;

        if langs.is_empty() {
            return Err(anyhow::anyhow!("No parsers found"));
        }

        Ok(langs)
    }
}

#[async_trait::async_trait]
impl Subcommand for Install {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler: Box<dyn Compiler> = self.select_compiler();
        let mut state = State::new()?;
        let mut parsers = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&self.tag)?;
        parsers.fetch_list(&self.tag).await?;

        let destination = PATHS.ts_parsers.join("install-tmp");
        self.cleanup()?;

        let langs: &Vec<String> = &self.select_langs(&parsers)?;
        let tag = self.select_tag(&changelog);
        let (is_installed, _) = state.all_installed(langs);

        if !self.force && !is_installed.is_empty() {
            c_println!(amber, "Parsers are already installed: {:?}", is_installed);
            return Ok(());
        }

        let msg = match self.force {
            true => "Force installing parser",
            false => "Installing parser",
        };

        match self.method {
            ParserInstallMethod::Compile => {
                check_command_exists("pnpm")?;
                check_command_exists("tree-sitter")?;

                for (idx, lang) in langs.clone().iter().enumerate() {
                    c_println!(blue, "\n{}/{}. {msg} {lang}", (idx + 1), langs.len());
                    let parser = parsers.get_parser(lang).unwrap();
                    Parser::compile(lang, parser, &compiler, &self.target, &destination).await?;
                    state.add_parser(lang, &tag, ParserInstallMethod::Compile, parser);
                }
            }
            // TODO: Implement download method
            ParserInstallMethod::Download => {
                todo!("beep boop beep boop! I'm a robot!");
            }
        }

        if self.force {
            Backups::create_backup(&mut state, &format!("{tag}-force"))?;
        }
        copy_all(destination, PATHS.ts_parsers.join("parsers"))?;

        state.commit()?;
        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        let destination = PATHS.ts_parsers.join("install-tmp");
        if destination.exists() {
            remove_all(&destination)?;
        }
        Ok(())
    }
}

impl Drop for Install {
    fn drop(&mut self) {
        self.cleanup().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    use crate::utils::fs::remove_all;

    fn setup() {
        let cwd = std::env::current_dir().unwrap();
        let test_assets = cwd.join("test-assets");

        let paths = [
            test_assets.join("backups"),
            test_assets.join("parsers"),
            test_assets.join("state.json"),
            test_assets.join("wanted-parsers.txt"),
        ];
        for path in paths.iter() {
            if path.exists() {
                remove_all(path).unwrap();
            }
        }

        std::fs::copy(
            test_assets.join("state-tmpl.json"),
            test_assets.join("state.json"),
        )
        .unwrap();
        std::fs::write(
            test_assets.join("wanted-parsers.txt"),
            "rust\nlua\nblueprint\nmarkdown",
        )
        .unwrap();
    }

    #[test]
    fn test_install_specific() {
        setup();
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_TEST", "true");
        cmd.args(["install", "rust", "lua", "blueprint", "markdown"]);
        cmd.assert().success();
    }

    #[test]
    fn test_install_wanted() {
        setup();
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_TEST", "true");
        cmd.args(["install", "--wanted"]);
        cmd.assert().success();
    }

    #[test]
    fn test_install_with_zig() {
        setup();
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_TEST", "true");
        cmd.args([
            "install",
            "--compiler",
            "zig",
            "--target",
            "x86_64-windows",
            "rust",
        ]);
        cmd.assert().success();
    }

    #[test]
    fn test_force_install() {
        setup();
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_TEST", "true");
        cmd.args(["install", "--wanted"]);
        cmd.assert().success();

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_TEST", "true");
        cmd.args(["install", "--wanted", "--force"]);
        cmd.assert().success();
    }
}