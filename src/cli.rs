use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

use crate::subcommands::{Compile, Install, Subcommand};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .valid(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Debug, Parser)]
#[clap(version, about, styles = STYLES)]
pub enum Cli {
    /// Compile tree-sitter parsers with zig or clang to a target directory
    #[clap(name = "compile")]
    Compile(Compile),

    /// Compile/Download tree-sitter parsers and install them
    #[clap(name = "install")]
    Install(Install),
    // /// Uninstall tree-sitter parsers
    // #[clap(name = "uninstall")]
    // Uninstall(Uninstall),

    // /// List tree-sitter parsers
    // #[clap(name = "list")]
    // List,

    // /// Restore to a previous release of tree-sitter parsers
    // #[clap(name = "restore")]
    // Restore,
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Compile(cmd) => cmd.run().await?,
            Self::Install(cmd) => cmd.run().await?,
            _ => {}
        }
        Ok(())
    }

    // pub fn cleanup(&self) -> anyhow::Result<()> {
    //     match self {
    //         Self::Compile(cmd) => cmd.cleanup()?,
    //         Self::Install(cmd) => cmd.cleanup()?,
    //         _ => {}
    //     }
    //     Ok(())
    // }
}

// impl Drop for Cli {
//     fn drop(&mut self) {
//         // self.cleanup().unwrap();
//         match self {
//             Self::Compile(cmd) => cmd.cleanup().unwrap(),
//             Self::Install(cmd) => cmd.cleanup().unwrap(),
//             _ => {}
//         }
//     }
// }
