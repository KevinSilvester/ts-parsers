use std::path::PathBuf;

use crate::{
    c_println,
    compiler::{Clang, Compiler, Zig, ZigTargets},
    data::{changelog::ChangeLog, parsers::Parsers},
    parser::Parser,
    utils::command::check_command_exists,
};

use super::Subcommand;

#[derive(Debug, Default, Clone, clap::ValueEnum)]
enum Compilers {
    #[default]
    Clang,
    Zig,
}

#[derive(Debug, Default, Clone, clap::ValueEnum)]
enum InstallMethods {
    #[default]
    Copmile,
    Download,
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

impl Compile {
    fn select_compiler(&self) -> Box<dyn Compiler> {
        match self.compiler {
            Compilers::Clang => Box::new(Clang::new()),
            Compilers::Zig => Box::new(Zig::new()),
        }
    }

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
}

#[async_trait::async_trait]
impl Subcommand for Compile {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler = self.select_compiler();
        let mut parsers = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&self.tag)?;
        parsers.fetch_list(&self.tag).await?;

        let langs = &self.select_langs(&parsers)?;
        Parser::check_compile_deps(&compiler)?;

        for (idx, lang) in langs.clone().iter().enumerate() {
            c_println!(
                blue,
                "\n{}/{}. Compiling parser {lang}",
                (idx + 1),
                langs.len()
            );
            let parser = parsers.get_parser(lang).unwrap();
            Parser::compile(lang, parser, &compiler, &self.target, &self.destination).await?;
        }
        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
