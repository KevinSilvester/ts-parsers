mod backups;
mod compile;
mod install;
mod lock;
mod uninstall;
mod unlock;
mod update;

#[async_trait::async_trait]
pub trait Subcommand {
    async fn run(&self) -> anyhow::Result<()>;
}

pub trait Langs {
    fn select_langs(
        &self,
        all: bool,
        wanted: bool,
        parser_names: &[String],
        parsers: &Parsers,
    ) -> anyhow::Result<Vec<String>> {
        if all {
            return Ok(parsers.langs.clone());
        }
        let langs = match wanted {
            true => {
                if parsers.wanted.is_none() {
                    c_println!(red, "No wanted parsers found");
                    return Err(anyhow::anyhow!("No wanted parsers found"));
                }
                parsers.wanted.clone().unwrap()
            }
            false => Vec::from(parser_names),
        };
        parsers.validate_parsers(&langs)?;
        if langs.is_empty() {
            return Err(anyhow::anyhow!("No parsers found"));
        }
        Ok(langs)
    }
}

use crate::c_println;
use crate::data::parsers::Parsers;

pub use self::backups::Backups;
pub use self::compile::Compile;
pub use self::install::Install;
pub use self::lock::Lock;
pub use self::uninstall::Uninstall;
pub use self::unlock::Unlock;
pub use self::update::Update;
