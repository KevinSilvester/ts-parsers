use std::{
    collections::VecDeque,
    io::{self, IsTerminal},
    path::Path,
    println,
    process::Stdio,
};

use strip_ansi_escapes::strip_str;
use tokio::process::Command;
use tokio_process_stream::{Item, ProcessLineStream};
use tokio_stream::StreamExt;

use crate::utils::{
    colors::{BLUE, GREEN, RED, TURQUOISE},
    renderer::Renderer,
};

const MAX_LINES: usize = 20;

pub fn check_exists(command: &str) -> anyhow::Result<()> {
    match std::process::Command::new(command)
        .stdout(Stdio::null())
        .output()
    {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound | std::io::ErrorKind::PermissionDenied => {
                anyhow::bail!("Command '{command}' not found: {error}")
            }
            _ => Ok(()),
        },
    }
}

pub async fn run(name: &str, args: &[&str], cwd: Option<impl AsRef<Path>>) -> anyhow::Result<bool> {
    let mut command = Command::new(name);
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    let stdin = io::stdin();
    let mut renderer = Renderer::new(std::io::stdout());
    let mut out_queue: VecDeque<String> = VecDeque::with_capacity(MAX_LINES);
    let mut is_finished = false;
    let mut failed = false;

    println!(
        "{TURQUOISE}=>{TURQUOISE:#} {} {} {BLUE}(running...){BLUE:#}",
        name,
        args.join(" ")
    );

    if stdin.is_terminal() {
        let mut procstream = ProcessLineStream::try_from(command)?;

        for _ in 0..MAX_LINES {
            if let Some(item) = procstream.next().await {
                match item {
                    Item::Done(status) => {
                        is_finished = true;
                        failed = !status.unwrap().success();
                        break;
                    }
                    Item::Stdout(out) => {
                        out_queue.push_back(format!("   {BLUE}=>{BLUE:#} {}", strip_str(&out)));
                    }
                    Item::Stderr(err) => {
                        out_queue.push_back(format!("   {RED}=>{RED:#} {}", strip_str(&err)));
                    }
                }
                renderer.render_queue(&out_queue)?;
            } else {
                is_finished = true;
                break;
            }
        }

        if !is_finished {
            while let Some(item) = procstream.next().await {
                out_queue.pop_front();
                match item {
                    Item::Done(status) => {
                        failed = !status.unwrap().success();
                        break;
                    }
                    Item::Stdout(out) => {
                        out_queue.push_back(format!("   {BLUE}=>{BLUE:#} {}", strip_str(&out)));
                    }
                    Item::Stderr(err) => {
                        out_queue.push_back(format!("   {RED}=>{RED:#} {}", strip_str(&err)));
                    }
                }
                renderer.render_queue(&out_queue)?;
            }
        }

        renderer.clear_output_final()?;
    } else {
        let output = command.output().await?;
        failed = !output.status.success();
    }

    if failed {
        eprintln!(
            "{RED}=>{RED:#} {} {} {RED}(failed){RED:#}",
            name,
            args.join(" ")
        );
        return Ok(false);
    }

    println!(
        "{TURQUOISE}=>{TURQUOISE:#} {} {} {GREEN}(complete!){GREEN:#}",
        name,
        args.join(" ")
    );

    Ok(true)
}
