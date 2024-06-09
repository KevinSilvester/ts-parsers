mod cc;
mod zig;

use self::cc::CC;
use self::zig::Zig;
use std::path::Path;

pub use self::zig::ZigTargets;

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
pub enum CompilerOption {
    #[default]
    Clang,
    Gcc,
    Zig,
}

pub fn select_compiler(compiler: &CompilerOption) -> Box<dyn Compiler> {
    match compiler {
        CompilerOption::Clang => Box::new(CC::new(CC::CLANG)),
        CompilerOption::Gcc => Box::new(CC::new(CC::GCC)),
        CompilerOption::Zig => Box::new(Zig::new()),
    }
}
