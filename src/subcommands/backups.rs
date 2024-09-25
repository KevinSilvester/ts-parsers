use tabled::{builder::Builder as TableBuilder, settings::Style as TableStyle};

use crate::{c_println, data::state::State, ops::backups_ops, utils::num_args};

use super::Subcommand;

#[derive(Debug, clap::Args)]
pub struct Backups {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Create a new backup
    Create,

    /// Restore from a selected restore-point
    Restore {
        /// ID of the backup to restore
        id: i32,
    },

    /// List all available backups
    List,

    /// Delete backups
    Delete {
        /// IDs of the backups to delete.
        /// Input can be a single ID, a space-separated list of IDs or a range of IDs.
        ///
        /// eg.
        /// - '1' (delete backup with ID 1)
        /// - '1 2 3' (delete backups with IDs 1, 2 and 3)
        /// - '1..3' (delete backups with IDs 1 and 2)
        /// - '1 2..=4' (delete backups with IDs 1, 2, 3 and 4)
        #[clap(conflicts_with = "all", verbatim_doc_comment)]
        ids: Option<String>,

        /// Delete all backups
        #[clap(short, long, conflicts_with = "ids")]
        all: bool,
    },
}

#[async_trait::async_trait]
impl Subcommand for Backups {
    async fn run(&self) -> anyhow::Result<()> {
        let mut state = State::new()?;

        match &self.commands {
            Commands::Create => {
                c_println!(blue, "Creating backup...");
                backups_ops::create_backup(&mut state, "manual")?;
                state.commit()?;
                c_println!(green, "Backup created!");

                Ok(())
            }
            Commands::List => {
                let backups = state.list_restore_points()?;
                if backups.is_empty() {
                    c_println!(amber, "No backups found");
                    return Ok(());
                }

                let mut builder = TableBuilder::default();
                builder.push_record(["ID", "Date", "File", "FileSize"]);

                for (i, (date, file, size)) in backups.iter().enumerate() {
                    builder.push_record([
                        (i + 1).to_string(),
                        date.to_owned(),
                        file.to_owned(),
                        size.to_owned(),
                    ]);
                }

                let mut table = builder.build();
                table.with(TableStyle::rounded());

                println!("{}", table);

                Ok(())
            }
            Commands::Restore { id } => {
                let mut state = State::new()?;
                let id = id - 1;

                if id < 0 {
                    anyhow::bail!("ID must be a positive integer");
                }

                if !state.check_restore_exists(id as usize)? {
                    anyhow::bail!("Backup with ID {} does not exist", id + 1);
                }

                c_println!(blue, "Restoring backup...");
                backups_ops::restore_backup(&mut state, id as usize)?;
                state.commit()?;
                c_println!(green, "Backup restored!");

                Ok(())
            }
            Commands::Delete { ids, all } => {
                if *all {
                    c_println!(blue, "Deleting all backups...");
                    if state.restore_points.is_empty() {
                        c_println!(amber, "No backups to delete");
                        return Ok(());
                    }

                    for id in 0..state.restore_points.len() {
                        backups_ops::delete_backup(&mut state, id)?;
                    }

                    state.commit()?;
                    c_println!(green, "All backups deleted!");
                    return Ok(());
                }

                match ids {
                    Some(ids_str) => {
                        let ids = num_args::parse_args(ids_str)?;
                        dbg!(ids);
                    }
                    None => {
                        anyhow::bail!("You must provide an ID or range of IDs to delete");
                    }
                }

                Ok(())
            }
        }
    }
}
