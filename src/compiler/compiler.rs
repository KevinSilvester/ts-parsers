use std::path::Path;

use super::zig::ZigTargets;

#[async_trait::async_trait]
pub trait Compiler: Send + Sync {
    async fn compile(
        &self,
        files: &[&str],
        cwd: &Path,
        target: &Option<ZigTargets>,
    ) -> anyhow::Result<()>;
}
