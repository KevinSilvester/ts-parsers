mod backups;
mod compile;
mod install;
mod lock;
mod uninstall;
mod unlock;
mod update;

#[async_trait::async_trait]
pub trait Subcommand {
    async fn run(&self) -> anyhow::Result<()>;
}

pub use self::backups::Backups;
pub use self::compile::Compile;
pub use self::install::Install;
pub use self::lock::Lock;
pub use self::uninstall::Uninstall;
pub use self::unlock::Unlock;
pub use self::update::Update;
