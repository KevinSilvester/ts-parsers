use std::path::Path;

use crate::utils::command;

use super::Compiler;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ZigTargets {
    #[clap(name = "x86_64-linux")]
    X86_64Linux,

    #[clap(name = "aarch64-linux")]
    Aarch64Linux,

    #[clap(name = "x86_64-macos")]
    X86_64MacOS,

    #[clap(name = "aarch64-macos")]
    Aarch64MacOS,

    #[clap(name = "x86_64-windows")]
    X86_64Windows,
}

#[derive(Debug)]
pub struct Zig<'a> {
    command: &'a str,
    base_args: &'a [&'a str],
}

impl<'a> Zig<'a> {
    pub fn new() -> Self {
        Self {
            command: "zig",
            base_args: &["c++", "-o", "out.so", "-lc", "-shared", "-Os"],
        }
    }

    fn build_args(&self, files: &'a [&'a str], target: &Option<ZigTargets>) -> Vec<&'a str> {
        let mut args = vec![];
        args.extend(self.base_args);
        args.push("-Isrc");
        args.extend(files);

        match target {
            Some(ZigTargets::X86_64Linux) => {
                args.push("-target");
                args.push("x86_64-linux");
            }
            Some(ZigTargets::Aarch64Linux) => {
                args.push("-target");
                args.push("aarch64-linux");
            }
            Some(ZigTargets::X86_64MacOS) => {
                args.push("-target");
                args.push("x86_64-macos");
            }
            Some(ZigTargets::Aarch64MacOS) => {
                args.push("-target");
                args.push("aarch64-macos");
            }
            Some(ZigTargets::X86_64Windows) => {
                args.push("-target");
                args.push("x86_64-windows");
            }
            None => {}
        }
        args
    }
}

#[async_trait::async_trait]
impl Compiler for Zig<'_> {
    fn get_name(&self) -> &str {
        "zig"
    }

    async fn compile(
        &self,
        files: &[&str],
        cwd: &Path,
        target: &Option<ZigTargets>,
    ) -> anyhow::Result<()> {
        command::check_exists("zig")?;
        let args = self.build_args(files, target);
        match command::run(self.command, &args, Some(cwd)).await? {
            true => Ok(()),
            false => anyhow::bail!("Failed to compile files"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_args_with_target() {
        let zig = Zig::new();
        let files = vec!["src/scanner.c", "src/parser.c"];
        let target = Some(ZigTargets::X86_64Linux);
        let args = zig.build_args(&files, &target);
        assert_eq!(
            args,
            vec![
                "c++",
                "-o",
                "out.so",
                "-lc",
                "-shared",
                "-Os",
                "-Isrc",
                "src/scanner.c",
                "src/parser.c",
                "-target",
                "x86_64-linux"
            ]
        );
    }

    #[test]
    fn test_build_args_without_target() {
        let zig = Zig::new();
        let files = vec!["src/scanner.c", "src/parser.c"];
        let target = None;
        let args = zig.build_args(&files, &target);
        assert_eq!(
            args,
            vec![
                "c++",
                "-o",
                "out.so",
                "-lc",
                "-shared",
                "-Os",
                "-Isrc",
                "src/scanner.c",
                "src/parser.c"
            ]
        );
    }
}
