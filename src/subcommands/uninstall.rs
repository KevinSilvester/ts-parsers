use crate::{
    c_println,
    data::{parsers::Parsers, state::State},
    ops::{backups_ops, parser_ops},
    utils::PATHS,
};

use super::Subcommand;

#[derive(Debug, clap::Args)]
pub struct Uninstall {
    /// Uninstall all installed parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// Uninstall parsers in 'wanted-parsers.txt'.
    /// This will only uninstall parsers that are already installed.
    ///
    /// Cannot be used with --all or [PARSERS...]
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"], verbatim_doc_comment)]
    wanted: bool,

    /// Uninstall specific parsers (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,
}

impl Uninstall {
    fn select_langs(&self, parsers: &Parsers, state: &State) -> anyhow::Result<Vec<String>> {
        if self.all {
            return Ok(state.parsers.keys().cloned().collect());
        }

        let langs = match self.wanted {
            true => {
                if parsers.wanted.is_none() {
                    anyhow::bail!("No wanted parsers found");
                }
                parsers.wanted.clone().unwrap()
            }
            false => self.parsers.clone(),
        };

        if langs.is_empty() {
            anyhow::bail!("No parsers found");
        }

        Ok(langs)
    }
}

#[async_trait::async_trait]
impl Subcommand for Uninstall {
    async fn run(&self) -> anyhow::Result<()> {
        let mut state = State::new()?;
        let parsers = Parsers::new()?;

        let langs = self.select_langs(&parsers, &state)?;
        let (is_installed, not_installed) = state.check_all_installed(&langs);
        let to_uninstall = &is_installed
            .iter()
            .filter(|lang| !state.is_locked(lang))
            .collect::<Vec<_>>();
        let is_locked = &is_installed
            .iter()
            .filter(|lang| state.is_locked(lang))
            .collect::<Vec<_>>();

        if !is_locked.is_empty() {
            c_println!(amber, "Parsers are locked: {:?}", is_locked);
        }
        if !self.all && !not_installed.is_empty() {
            anyhow::bail!("Parsers are not installed: {:?}", not_installed);
        }
        if to_uninstall.is_empty() {
            c_println!(blue, "No parsers to uninstall!");
            return Ok(());
        }

        backups_ops::create_backup(&mut state, "uninstall")?;

        for (idx, lang) in is_installed.iter().enumerate() {
            c_println!(
                blue,
                "\n{}/{}. Uninstalling parser {lang}",
                (idx + 1),
                is_installed.len()
            );
            parser_ops::uninstall(lang, &PATHS.ts_parsers.join("parser"))?;
            state.remove_parser(lang);
        }

        state.commit()?;

        Ok(())
    }
}
