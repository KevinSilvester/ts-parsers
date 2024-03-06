use clap::builder::{
    styling::{AnsiColor, Effects},
    Styles,
};

use crate::subcommands::{Backups, Compile, Install, Lock, Subcommand, Uninstall, Unlock, Update};

fn style() -> Styles {
    Styles::default()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
}

#[derive(Debug, clap::Parser)]
#[clap(version, about, styles = style())]
pub enum Cli {
    /// Create/restore backups of installed tree-sitter parsers
    #[clap(name = "backups")]
    Backups(Backups),

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
    Lock(Lock),

    /// Self update
    #[clap(name = "self-update")]
    SelfUpdate,

    /// List all tree-sitter parsers (installed and available)
    #[clap(name = "parsers")]
    Parsers,

    /// Uninstall tree-sitter parsers
    #[clap(name = "uninstall")]
    Uninstall(Uninstall),

    /// Update installed tree-sitter parsers
    #[clap(name = "update")]
    Update(Update),

    /// Unlock the current state of installed tree-sitter parsers
    #[clap(name = "unlock")]
    Unlock(Unlock),
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Compile(cmd) => cmd.run().await?,
            Self::Install(cmd) => cmd.run().await?,
            Self::Update(cmd) => cmd.run().await?,
            Self::Uninstall(cmd) => cmd.run().await?,
            Self::Lock(cmd) => cmd.run().await?,
            Self::Unlock(cmd) => cmd.run().await?,
            Self::Backups(cmd) => cmd.run().await?,
            Self::Changelog => todo!(),
            Self::Clean => todo!(),
            Self::SelfUpdate => todo!(),
            Self::Parsers => todo!(),
        }
        Ok(())
    }
}
