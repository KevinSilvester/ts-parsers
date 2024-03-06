use std::{collections::VecDeque, path::Path, println, process::Stdio};

use ansi_term::Colour;
use lazy_static::lazy_static;
use strip_ansi_escapes::strip_str;
use tokio::process::Command;
use tokio_process_stream::{Item, ProcessLineStream};
use tokio_stream::StreamExt;

use crate::utils::renderer::Renderer;

const MAX_LINES: usize = 20;

lazy_static! {
    static ref TURQUOISE: Colour = Colour::RGB(66, 242, 245);
    static ref BLUE: Colour = Colour::RGB(2, 149, 235);
    static ref RED: Colour = Colour::RGB(235, 66, 66);
    static ref GREEN: Colour = Colour::RGB(57, 219, 57);
}

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

    let mut renderer = Renderer::new(std::io::stdout());
    let mut out_queue: VecDeque<String> = VecDeque::with_capacity(MAX_LINES);
    let mut is_finished = false;
    let mut failed = false;

    println!(
        "{} {} {} {}",
        TURQUOISE.paint("=>"),
        name,
        args.join(" "),
        BLUE.paint("(running...)")
    );
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
                    out_queue.push_back(format!("   {} {}", BLUE.paint("=>"), strip_str(&out)));
                }
                Item::Stderr(err) => {
                    out_queue.push_back(format!("   {} {}", RED.paint("=>"), strip_str(&err)));
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
                    out_queue.push_back(format!("   {} {}", BLUE.paint("=>"), strip_str(&out)));
                }
                Item::Stderr(err) => {
                    out_queue.push_back(format!("   {} {}", RED.paint("=>"), strip_str(&err)));
                }
            }
            renderer.render_queue(&out_queue)?;
        }
    }

    renderer.clear_output_final()?;

    if failed {
        eprintln!(
            "{} {} {} {}",
            RED.paint("=>"),
            name,
            args.join(" "),
            RED.paint("(failed!)")
        );
        return Ok(false);
    }

    println!(
        "{} {} {} {}",
        TURQUOISE.paint("=>"),
        name,
        args.join(" "),
        GREEN.paint("(complete!)")
    );

    Ok(true)
}
