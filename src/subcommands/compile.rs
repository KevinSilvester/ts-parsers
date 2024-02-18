use std::path::PathBuf;

use lazy_static::lazy_static;

use crate::{
    c_println,
    compiler::{Clang, Compiler, Zig, ZigTargets},
    data::{changelog::ChangeLog, parsers::Parsers, state::State},
    parser::Parser,
    utils::Paths,
};

use super::Subcommand;

lazy_static! {
    static ref PATHS: Paths = Paths::new();
}

#[derive(Debug, Default, Clone, clap::ValueEnum)]
enum Compilers {
    #[default]
    Clang,
    Zig,
}

#[derive(Debug, clap::Args)]
pub struct Compile {
    /// Compiler to use
    #[clap(short, long, default_value_t, value_enum)]
    compiler: Compilers,

    /// Zig target, only used when compiler is zig
    #[clap(short, long, value_enum)]
    target: Option<ZigTargets>,

    /// Compile all parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// Destination to install the compiled parsers
    #[clap(short, long)]
    destination: PathBuf,

    /// `nvim-treesitter-parsers` release to use
    #[clap(long)]
    tag: Option<String>,

    /// Compile parsers in `wanted` list
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"] )]
    wanted: bool,

    /// Parsers to compile (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,
}

#[async_trait::async_trait]
impl Subcommand for Compile {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler: Box<dyn Compiler> = match self.compiler {
            Compilers::Clang => Box::new(Clang::new()),
            Compilers::Zig => Box::new(Zig::new()),
        };
        let state = State::new()?;
        let mut parsers = Parsers::new();
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        let tag = match &self.tag {
            Some(tag) => tag.clone(),
            None => changelog.get_latest_tag().unwrap(),
        };
        changelog.check_entry(&tag)?;
        parsers.fetch_list(Some(tag.clone())).await?;

        if self.all {
            for lang in &parsers.langs {
                let parser = parsers.get_parser(lang).unwrap();
                c_println!(blue, "\nCompiling {lang} parser");
                Parser::compile(lang, parser, &compiler, &self.target, &self.destination).await?;
            }
        } else if self.wanted {
            if state.list_wanted().is_none() {
                c_println!(red, "No wanted parsers found");
                return Err(anyhow::anyhow!("No wanted parsers found"));
            }

            parsers.validate_parsers(state.list_wanted().unwrap(), &tag)?;

            for lang in state.list_wanted().unwrap() {
                let parser = parsers.get_parser(lang).unwrap();
                c_println!(blue, "\nCompiling {lang} parser");
                Parser::compile(lang, parser, &compiler, &self.target, &self.destination).await?;
            }
        } else {
            parsers.validate_parsers(&self.parsers, &tag)?;

            for lang in &self.parsers {
                let parser = parsers.get_parser(lang).unwrap();
                c_println!(blue, "\nCompiling {lang} parser");
                Parser::compile(lang, parser, &compiler, &self.target, &self.destination).await?;
            }
        }

        Ok(())
    }
}
