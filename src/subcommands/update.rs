use crate::{
    c_println,
    compiler::{select_compiler, CompilerOption},
    data::{
        changelog::ChangeLog,
        parsers::Parsers,
        state::{ParserInstallMethod, State},
    },
    ops::{
        backups_ops,
        parser_ops::{self, NodePackageManagers},
    },
    utils::{colors::TURQUOISE, fs as ufs, PATHS},
};

use super::{Langs, Subcommand};

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

    /// Node package manager to use when compiling from grammar
    /// Bun is only available on unix systems
    #[clap(short, long, default_value_t, value_enum, verbatim_doc_comment)]
    npm: NodePackageManagers,

    /// Generate the parser from grammar
    #[clap(short, long, default_value = "false")]
    from_grammar: bool,
}

impl Update {
    fn cleanup(&self) -> anyhow::Result<()> {
        let destination = PATHS.ts_parsers.join(".update-tmp");
        if destination.exists() {
            ufs::remove_all(&destination)?;
        }
        Ok(())
    }
}

impl Langs for Update {}

#[derive(Debug)]
enum UpdateType {
    TagOnly,
    TagAndParser,
}

#[async_trait::async_trait]
impl Subcommand for Update {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler = &*select_compiler(&self.compiler);
        let mut state = State::new()?;
        let mut parsers = Parsers::new()?;
        let mut parsers_previous = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&None)?;
        parsers.fetch_list(&None).await?;
        parsers_previous
            .fetch_list(&changelog.get_previous_tag())
            .await?;

        let destination = PATHS.ts_parsers.join(".update-tmp");
        self.cleanup()?;

        let langs = &self.select_langs(self.all, self.wanted, &self.parsers, &parsers)?;
        let tag = changelog.get_latest_tag().unwrap();
        let (is_installed, not_installed) = state.check_all_installed(langs);

        if is_installed.is_empty() {
            c_println!(amber, "Selected parsers are not installed!");
            return Ok(());
        }

        if !self.all && !not_installed.is_empty() {
            c_println!(amber, "Parsers are not installed: {:?}", not_installed);
        }

        let is_locked = &is_installed
            .iter()
            .filter(|lang| state.is_locked(lang))
            .collect::<Vec<_>>();
        let up_to_date = &is_installed
            .iter()
            .filter(|lang| state.is_tag_up_to_date(lang, &tag))
            .collect::<Vec<_>>();
        let to_update = &is_installed
            .iter()
            .filter(|lang| !state.is_tag_up_to_date(lang, &tag) && !state.is_locked(lang))
            .map(|lang| {
                let update_type = match parsers_previous.get_parser(lang).unwrap().revision
                    == parsers.get_parser(lang).unwrap().revision
                {
                    true => UpdateType::TagOnly,
                    false => UpdateType::TagAndParser,
                };
                (lang, update_type)
            })
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
        let mut parsers_compiled = false;

        match self.method {
            ParserInstallMethod::Compile => {
                parser_ops::check_compile_deps(compiler, &self.npm)?;

                for (idx, (lang, update_type)) in to_update.iter().enumerate() {
                    c_println!(
                        blue,
                        "\n{}/{}. Updating parser {lang}",
                        (idx + 1),
                        to_update.len()
                    );
                    let parser = parsers.get_parser(lang).unwrap();

                    match update_type {
                        UpdateType::TagOnly => {
                            let install_method = state.parsers.get(*lang).unwrap().install_method;
                            println!("{TURQUOISE}->{TURQUOISE:#} updating parser data");
                            state.update_parser(lang, &tag, install_method, parser);
                        }
                        UpdateType::TagAndParser => {
                            parser_ops::compile(
                                lang,
                                parser,
                                compiler,
                                &None,
                                &self.npm,
                                self.from_grammar,
                                &destination,
                            )
                            .await?;
                            state.update_parser(lang, &tag, ParserInstallMethod::Compile, parser);
                            parsers_compiled = true;
                        }
                    }
                }
            }
            // TODO: Implement download method
            ParserInstallMethod::Download => {
                todo!("beep boop beep boop! I'm a robot!");
            }
        }

        if parsers_compiled {
            ufs::copy_all(&destination, PATHS.ts_parsers.join("parser"))?;
        }
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
