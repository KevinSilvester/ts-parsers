use std::path::PathBuf;

use crate::{
    c_println,
    compiler::{CompilerOption, ZigTargets, select_compiler},
    data::{changelog::ChangeLog, parsers::Parsers},
    ops::parser_ops::{self, NodePackageManagers},
};

use super::{Langs, Subcommand};

#[derive(Debug, clap::Args)]
pub struct Compile {
    /// Compiler to use
    #[clap(short, long, default_value_t, value_enum)]
    compiler: CompilerOption,

    /// Zig compile target
    /// Only used when compiler is zig.
    /// (defaults to host architecture)
    #[clap(short, long, value_enum, verbatim_doc_comment)]
    target: Option<ZigTargets>,

    /// Compile all parsers
    #[clap(short, long, default_value = "false")]
    all: bool,

    /// Generate the parser from grammar
    #[clap(short, long, default_value = "false")]
    from_grammar: bool,

    /// Node package manager to use when compiling from grammar
    /// Bun is only available on unix systems
    #[clap(short, long, default_value_t, value_enum, verbatim_doc_comment)]
    npm: NodePackageManagers,

    /// Output directory to compile parsers to
    #[clap(short, long)]
    destination: PathBuf,

    /// 'nvim-treesitter-parsers' tags to use.
    /// Will only use tags present in the changelog.
    /// (defaults to latest tag)
    ///
    /// See https://github.com/KevinSilvester/nvim-treesitter-parerers
    #[clap(long, verbatim_doc_comment)]
    tag: Option<String>,

    /// Compile parsers in 'wanted-parsers.txt'
    #[clap(short, long, default_value = "false", conflicts_with_all = ["all", "parsers"] )]
    wanted: bool,

    /// Parsers to compile (cannot be used with --all or --wanted)
    #[clap(conflicts_with = "all")]
    parsers: Vec<String>,

    /// Continue compiling even if a parser fails to compile
    #[clap(long, default_value = "false")]
    no_fail_fast: bool,
}

impl Langs for Compile {}

#[async_trait::async_trait]
impl Subcommand for Compile {
    async fn run(&self) -> anyhow::Result<()> {
        let compiler = &*select_compiler(&self.compiler);
        let mut parsers = Parsers::new()?;
        let mut changelog = ChangeLog::new();

        changelog.fetch_changelog().await?;
        changelog.check_entry(&self.tag)?;
        parsers.fetch_list(&self.tag).await?;

        let langs = &self.select_langs(self.all, self.wanted, &self.parsers, &parsers)?;
        parser_ops::check_compile_deps(compiler, &self.npm)?;

        let mut failed_langs = vec![];

        for (idx, lang) in langs.iter().enumerate() {
            c_println!(
                blue,
                "\n{}/{}. Compiling parser {lang}",
                (idx + 1),
                langs.len()
            );
            let parser = parsers.get_parser(lang).unwrap();
            match parser_ops::compile(
                lang,
                parser,
                compiler,
                &self.target,
                &self.npm,
                self.from_grammar,
                &self.destination,
            )
            .await
            {
                Ok(_) => (),
                Err(e) if self.no_fail_fast => {
                    c_println!(amber, "[WARNING]: \"{lang}\" - {e}");
                    failed_langs.push(lang);
                }
                Err(e) => return Err(e),
            };
        }

        if !failed_langs.is_empty() {
            c_println!(
                amber,
                "\n[WARNING]: Failed to compile parsers - {failed_langs:?}"
            );
        }

        Ok(())
    }
}
