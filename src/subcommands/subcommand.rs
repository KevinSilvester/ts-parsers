#[async_trait::async_trait]
pub trait Subcommand: Sized {
    async fn run(&self) -> anyhow::Result<()>;
}
