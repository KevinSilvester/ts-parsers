use crate::{
    c_println,
    data::{parsers::Parsers, state::State},
};

use super::Subcommand;

#[derive(Debug, clap::Args)]
pub struct Lock {
    /// Lock all installed parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// Lock parsers in `wanted-parsers.txt`.
    /// This will only uninstall parsers that are already installed.
    ///
    /// Cannot be used with --all or [PARSERS...]
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"], verbatim_doc_comment)]
    wanted: bool,

    /// Lock specific parsers (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,
}

impl Lock {
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
impl Subcommand for Lock {
    async fn run(&self) -> anyhow::Result<()> {
        let mut state = State::new()?;
        let parsers = Parsers::new()?;

        let langs = self.select_langs(&parsers, &state)?;
        let (is_installed, not_installed) = state.check_all_installed(&langs);

        if !not_installed.is_empty() {
            anyhow::bail!("Parsers are not installed: {:?}", not_installed);
        }

        for (idx, lang) in is_installed.iter().enumerate() {
            c_println!(
                blue,
                "\n{}/{}. Locking parser {lang}",
                (idx + 1),
                is_installed.len()
            );
            state.lock_parser(lang);
        }

        state.commit()?;

        Ok(())
    }
}
