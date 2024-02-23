mod clang;
mod zig;

use std::path::Path;

#[async_trait::async_trait]
pub trait Compiler: Send + Sync {
    async fn compile(
        &self,
        files: &[&str],
        cwd: &Path,
        target: &Option<ZigTargets>,
    ) -> anyhow::Result<()>;

    fn get_name(&self) -> &str;
}

#[derive(Debug, Default, Clone, clap::ValueEnum)]
pub enum Compilers {
    #[default]
    Clang,
    Zig,
}

pub use self::clang::Clang;
pub use self::zig::{Zig, ZigTargets};
