use crate::{
    c_println,
    compiler::{Compiler, Compilers, Zig, CC},
    data::{
        changelog::ChangeLog,
        parsers::Parsers,
        state::{ParserInstallMethod, State},
    },
    ops::{backups_ops, parser_ops},
    utils::{fs as ufs, PATHS},
};

use super::Subcommand;

#[derive(Debug, clap::Args)]
pub struct Install {
    /// Installation method
    #[clap(short, long, default_value_t, value_enum)]
    method: ParserInstallMethod,

    /// Compiler to use
    #[clap(short, long, default_value_t, value_enum)]
    compiler: Compilers,

    /// Compile all parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// `nvim-treesitter-parsers` tags to use
    ///
    /// Will only use tags present in the changelog
    /// (defaults to latest tag)
    ///
    /// See https://github.com/KevinSilvester/nvim-treesitter-parerers
    #[clap(long)]
    tag: Option<String>,

    /// Compile parsers in `wanted-parsers.txt`
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
            Compilers::Clang => Box::new(CC::new(CC::CLANG)),
            Compilers::Gcc => Box::new(CC::new(CC::GCC)),
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
        let compiler: &dyn Compiler = &*self.select_compiler();
        let mut state = State::new()?;
        let mut parsers = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&self.tag)?;
        parsers.fetch_list(&self.tag).await?;

        let destination = PATHS.ts_parsers.join(".install-tmp");
        self.cleanup()?;

        let langs = self.select_langs(&parsers)?;
        let tag = self.select_tag(&changelog);
        let (is_installed, _) = state.check_all_installed(&langs);

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
                parser_ops::check_compile_deps(compiler)?;

                for (idx, lang) in langs.iter().enumerate() {
                    c_println!(blue, "\n{}/{}. {msg} {lang}", (idx + 1), langs.len());
                    let parser = parsers.get_parser(lang).unwrap();
                    parser_ops::compile(lang, parser, compiler, &None, &destination).await?;
                    state.add_parser(lang, &tag, ParserInstallMethod::Compile, parser);
                }
            }
            // TODO: Implement download method
            ParserInstallMethod::Download => {
                todo!("beep boop beep boop! I'm a robot!");
            }
        }

        if self.force {
            backups_ops::create_backup(&mut state, &format!("{tag}-force"))?;
        }
        ufs::copy_all(destination, PATHS.ts_parsers.join("parsers"))?;

        state.commit()?;
        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        let destination = PATHS.ts_parsers.join(".install-tmp");
        if destination.exists() {
            ufs::remove_all(&destination)?;
        }
        Ok(())
    }
}

impl Drop for Install {
    fn drop(&mut self) {
        if self.cleanup().is_err() {
            c_println!(amber, "WARNING: Cleanup failed");
        }
    }
}
