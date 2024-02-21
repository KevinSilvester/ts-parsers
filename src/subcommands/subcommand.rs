#[async_trait::async_trait]
pub trait Subcommand {
    async fn run(&self) -> anyhow::Result<()>;
    fn cleanup(&self) -> anyhow::Result<()>;
}
