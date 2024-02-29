use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

use crate::subcommands::{Compile, Install, Subcommand, Update};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .valid(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Debug, Parser)]
#[clap(version, about, styles = STYLES)]
pub enum Cli {
    /// Create/restore backups of installed tree-sitter parsers
    #[clap(name = "backups")]
    Backups,

    /// Display changelog for nvim-treesitter-parsers
    #[clap(name = "changelog")]
    Changelog,

    /// Clean the download cache,backups and dangling compile artefacts
    #[clap(name = "clean")]
    Clean,

    /// Compile tree-sitter parsers with zig or clang to a target directory
    #[clap(name = "compile")]
    Compile(Compile),

    /// Compile/Download tree-sitter parsers and install them
    #[clap(name = "install")]
    Install(Install),

    /// Lock the current state of installed tree-sitter parsers
    #[clap(name = "lock")]
    Lock,

    /// Self update
    #[clap(name = "self-update")]
    SelfUpdate,

    /// List all tree-sitter parsers (installed and available)
    #[clap(name = "parsers")]
    Parsers,

    /// Uninstall tree-sitter parsers
    #[clap(name = "uninstall")]
    Uninstall,

    /// Update installed tree-sitter parsers
    #[clap(name = "update")]
    Update(Update),

    /// Unlock the current state of installed tree-sitter parsers
    #[clap(name = "unlock")]
    Unlock,
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Compile(cmd) => cmd.run().await?,
            Self::Install(cmd) => cmd.run().await?,
            Self::Update(cmd) => cmd.run().await?,
            _ => anyhow::bail!("Subcommand '{:?}' not implemented yet!", self),
        }
        Ok(())
    }
}
