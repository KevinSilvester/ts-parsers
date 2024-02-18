use std::{thread, time::Duration};

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

use crate::subcommands::{Compile, Subcommand};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .valid(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Debug, Parser)]
#[clap(version, about, styles = STYLES)]
pub enum Cli {
    /// Compile and optionally install tree-sitter parsers with zig or clang
    #[clap(name = "compile")]
    Compile(Compile),

    /// Download  and install pre-compiled tree-sitter parsers
    #[clap(name = "download")]
    Download,

    /// List tree-sitter parsers
    #[clap(name = "list")]
    List,

    /// Restore to a previous release of tree-sitter parsers
    #[clap(name = "restore")]
    Restore,
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Compile(cmd) => cmd.run().await?,
            Self::Download => {
                println!("Download");
            }
            Self::List => {
                println!("List");
            }
            Self::Restore => {
                println!("Restore");
            }
        }
        Ok(())
    }

    pub fn cleanup(&self) -> anyhow::Result<()> {
        // match self {
        // }
        Ok(())
    }
}
