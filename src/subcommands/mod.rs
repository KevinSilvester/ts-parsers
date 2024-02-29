mod compile;
mod install;
mod uninstall;
mod update;

#[async_trait::async_trait]
pub trait Subcommand {
    async fn run(&self) -> anyhow::Result<()>;
    fn cleanup(&self) -> anyhow::Result<()>;
}

pub use self::compile::Compile;
pub use self::install::Install;
pub use self::uninstall::Uninstall;
pub use self::update::Update;
