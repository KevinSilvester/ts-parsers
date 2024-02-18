use std::path::Path;

use crate::utils::command::run_command;

use super::{zig::ZigTargets, Compiler};

#[derive(Debug)]
pub struct Clang<'a> {
    command: &'a str,
    base_args: &'a [&'a str],
}

impl<'a> Clang<'a> {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        let base_args = &["-o", "out.so", "-Os", "-shared", "-fPIC"];

        #[cfg(target_os = "macos")]
        let base_args = &["-o", "out.so", "-Os", "-bundle", "-fPIC"];

        #[cfg(target_os = "windows")]
        let base_args = &["-o", "out.so", "-Os", "-shared"];

        Self {
            command: "clang",
            base_args,
        }
    }

    fn build_args(&self, files: &'a [&'a str]) -> Vec<&'a str> {
        let mut args = vec![];
        args.extend(self.base_args);

        if files
            .iter()
            .any(|file| file.ends_with(".cc") || file.ends_with(".cpp") || file.ends_with(".cxx"))
        {
            args.push("-lstdc++");
        }

        args.push("-Isrc");
        args.extend(files);
        args
    }
}

#[async_trait::async_trait]
impl Compiler for Clang<'_> {
    async fn compile(
        &self,
        files: &[&str],
        cwd: &Path,
        _: &Option<ZigTargets>,
    ) -> anyhow::Result<()> {
        let args = self.build_args(files);
        match run_command(self.command, &args, Some(cwd)).await? {
            true => Ok(()),
            false => anyhow::bail!("Failed to compile files"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_args() {
        let clang = Clang::new();
        let files = &["src/scanner.c", "src/parser.cc"];
        let args = clang.build_args(files);

        #[cfg(target_os = "linux")]
        let expected = vec![
            "-o",
            "out.so",
            "-Os",
            "-shared",
            "-fPIC",
            "-lstdc++",
            "-Isrc",
            "src/scanner.c",
            "src/parser.cc",
        ];

        #[cfg(target_os = "macos")]
        let expected = vec![
            "-o",
            "out.so",
            "-Os",
            "-bundle",
            "-fPIC",
            "-lstdc++",
            "-Isrc",
            "src/scanner.c",
            "src/parser.cc",
        ];

        #[cfg(target_os = "windows")]
        let expected = vec![
            "-o",
            "out.so",
            "-Os",
            "-shared",
            "-lstdc++",
            "-Isrc",
            "src/scanner.c",
            "src/parser.cc",
        ];
        assert_eq!(args, expected);
    }
}
