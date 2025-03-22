mod cli;
mod compiler;
mod data;
mod ops;
mod subcommands;
mod utils;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::cli::Cli;

use clap::Parser;
use tokio::{
    signal,
    sync::{Mutex, mpsc},
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

    tokio::spawn(async move {
        match cmd_c.lock().await.run().await {
            Ok(_) => shutdown_tx.send(Shutdown::Graceful).unwrap(),
            Err(err) => {
                c_println!(red, "[ERROR]: {}", err);
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

    match shutdown.load(Ordering::Relaxed) {
        true => {
            c_println!(green, "Gracefully shutting down... \\(￣︶￣*\\))");
            std::mem::drop(cmd);
            std::process::exit(0);
        }
        false => {
            // c_println!(red, "I borked... (┬┬﹏┬┬)");
            std::mem::drop(cmd);
            std::process::exit(1);
        }
    }
}
