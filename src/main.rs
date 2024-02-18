mod cli;
mod compiler;
mod data;
mod parser;
mod subcommands;
mod utils;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::cli::Cli;

use clap::Parser;
use tokio::{
    signal,
    sync::{mpsc, Mutex},
};

enum Shutdown {
    Graceful,
    Borked,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_c = Arc::clone(&shutdown);

    let cmd = Arc::new(Mutex::new(Cli::parse()));
    let cmd_c = Arc::clone(&cmd);

    let mut exit_code = 0;

    tokio::spawn(async move {
        match cmd_c.lock().await.run().await {
            Ok(_) => shutdown_tx.send(Shutdown::Graceful).unwrap(),
            Err(err) => {
                c_println!(red, "Error: {err}");
                shutdown_tx.send(Shutdown::Borked).unwrap()
            }
        }
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            c_println!(amber, "\nMISSION ABORTED!!!!!!!!! (╯°□°）╯");
            shutdown_c.store(true, Ordering::Relaxed);
        },
        shutdown = shutdown_rx.recv() => match shutdown.unwrap_or(Shutdown::Borked) {
            Shutdown::Graceful => shutdown_c.store(true, Ordering::Relaxed),
            Shutdown::Borked => shutdown_c.store(false, Ordering::Relaxed),
        },
    };

    std::mem::drop(cmd);

    match shutdown.load(Ordering::Relaxed) {
        true => c_println!(green, "Gracefully shutting down... \\(￣︶￣*\\))"),
        false => {
            c_println!(red, "I borked... (┬┬﹏┬┬)");
            exit_code = 1
        }
    }
    std::process::exit(exit_code);
}
