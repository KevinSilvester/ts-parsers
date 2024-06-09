use crate::{
    c_println,
    compiler::{select_compiler, CompilerOption},
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
pub struct Update {
    /// Installation method
    #[clap(short, long, default_value_t, value_enum)]
    method: ParserInstallMethod,

    /// Compiler to use
    #[clap(short, long, default_value_t, value_enum)]
    compiler: CompilerOption,

    /// Update all installed parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// Update parsers in 'wanted-parsers.txt'.
    /// This will only update parsers that are already installed.
    ///
    /// Cannot be used with --all or [PARSERS...]
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"], verbatim_doc_comment)]
    wanted: bool,

    /// Update specific (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,
}

impl Update {
    fn select_langs(&self, parsers: &Parsers) -> anyhow::Result<Vec<String>> {
        if self.all {
            return Ok(parsers.langs.clone());
        }

        let langs = match self.wanted {
            true => {
                if parsers.wanted.is_none() {
                    c_println!(red, "No wanted parsers found");
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

    fn cleanup(&self) -> anyhow::Result<()> {
        let destination = PATHS.ts_parsers.join(".update-tmp");
        if destination.exists() {
            ufs::remove_all(&destination)?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Subcommand for Update {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler = &*select_compiler(&self.compiler);
        let mut state = State::new()?;
        let mut parsers = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&None)?;
        parsers.fetch_list(&None).await?;

        let destination = PATHS.ts_parsers.join(".update-tmp");
        self.cleanup()?;

        let langs = self.select_langs(&parsers)?;
        let tag = changelog.get_latest_tag().unwrap();
        let (is_installed, not_installed) = state.check_all_installed(&langs);

        if is_installed.is_empty() {
            c_println!(amber, "Selected parsers are not installed!");
            return Ok(());
        }

        if !not_installed.is_empty() {
            c_println!(amber, "Parsers are not installed: {:?}", not_installed);
        }

        let is_locked = &is_installed
            .iter()
            .filter(|lang| state.is_locked(lang))
            .collect::<Vec<_>>();
        let up_to_date = &is_installed
            .iter()
            .filter(|lang| state.is_up_to_date(lang, &tag))
            .collect::<Vec<_>>();
        let to_update = &is_installed
            .iter()
            .filter(|lang| !state.is_up_to_date(lang, &tag) && !state.is_locked(lang))
            .collect::<Vec<_>>();

        if !is_locked.is_empty() {
            c_println!(amber, "Parsers are locked: {:?}", is_locked);
        }
        if !up_to_date.is_empty() {
            c_println!(blue, "Parsers are already up-to-date: {:?}", up_to_date);
        }
        if to_update.is_empty() {
            c_println!(blue, "No parsers to update!");
            return Ok(());
        }

        backups_ops::create_backup(&mut state, &format!("{tag}-update"))?;

        match self.method {
            ParserInstallMethod::Compile => {
                parser_ops::check_compile_deps(compiler)?;

                for (idx, lang) in to_update.iter().enumerate() {
                    c_println!(
                        blue,
                        "\n{}/{}. Updating parser {lang}",
                        (idx + 1),
                        to_update.len()
                    );
                    let parser = parsers.get_parser(lang).unwrap();
                    parser_ops::compile(lang, parser, compiler, &None, &destination).await?;
                    state.update_parser(lang, &tag, ParserInstallMethod::Compile, parser);
                }
            }
            // TODO: Implement download method
            ParserInstallMethod::Download => {
                todo!("beep boop beep boop! I'm a robot!");
            }
        }

        ufs::copy_all(&destination, PATHS.ts_parsers.join("parsers"))?;
        state.commit()?;

        Ok(())
    }
}

impl Drop for Update {
    fn drop(&mut self) {
        if self.cleanup().is_err() {
            c_println!(amber, "WARNING: Cleanup failed");
        }
    }
}
