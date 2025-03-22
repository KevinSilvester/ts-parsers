use crate::{
    c_println,
    compiler::{CompilerOption, select_compiler},
    data::{
        changelog::ChangeLog,
        parsers::Parsers,
        state::{ParserInstallMethod, State},
    },
    ops::{
        backups_ops,
        parser_ops::{self, NodePackageManagers},
    },
    utils::{PATHS, /* colors::TURQUOISE, */ fs as ufs},
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

        let mut is_locked = vec![];
        let mut up_to_date = vec![];
        let mut update_tags = vec![];
        let mut update_tags_and_parsers = vec![];

        for lang in &is_installed {
            if state.is_locked(lang) {
                is_locked.push(lang);
                continue;
            }

            if state.is_tag_up_to_date(lang, &tag) {
                up_to_date.push(lang);
                continue;
            }

            match state.parsers.get(lang).unwrap().revision
                == parsers.get_parser(lang).unwrap().revision
            {
                true => update_tags.push(lang),
                false => update_tags_and_parsers.push(lang),
            }
        }

        if !is_locked.is_empty() {
            c_println!(amber, "Parsers are locked: {:?}", is_locked);
        }
        if !up_to_date.is_empty() {
            c_println!(blue, "Parsers are already up-to-date: {:?}", up_to_date);
        }
        if update_tags_and_parsers.is_empty() {
            c_println!(blue, "No parsers to update!");
            return Ok(());
        }

        backups_ops::create_backup(&mut state, &format!("{tag}-update"))?;
        let mut parsers_compiled = false;

        for lang in update_tags {
            state.update_parser(
                lang,
                &tag,
                state.parsers.get(lang).unwrap().install_method,
                parsers.get_parser(lang).unwrap(),
            );
        }

        match self.method {
            ParserInstallMethod::Compile => {
                parser_ops::check_compile_deps(compiler, &self.npm)?;
                for (idx, lang) in update_tags_and_parsers.iter().enumerate() {
                    let parser = parsers.get_parser(lang).unwrap();

                    c_println!(
                        blue,
                        "\n{}/{}. Updating parser {lang}",
                        (idx + 1),
                        update_tags_and_parsers.len()
                    );
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
