use std::{fs::File, path::Path};

use chrono::Utc;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    c_println,
    data::state::{RestorePoint, State},
};

use super::PATHS;

#[derive(Debug)]
pub struct Backups;

fn zip_files(
    zip: &mut ZipWriter<&mut File>,
    options: &FileOptions,
    path: &Path,
) -> anyhow::Result<()> {
    if path.is_file() {
        let name = path.strip_prefix(&PATHS.ts_parsers)?.to_str().unwrap();
        zip.start_file(name, options.to_owned())?;
        let mut file = File::open(path)?;
        std::io::copy(&mut file, zip)?;
        return Ok(());
    }

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            zip_files(zip, options, &path)?;
        } else {
            let name = path.strip_prefix(&PATHS.ts_parsers)?.to_str().unwrap();
            zip.start_file(name, options.to_owned())?;
            let mut file = File::open(&path)?;
            std::io::copy(&mut file, zip)?;
        }
    }

    Ok(())
}

impl Backups {
    pub fn create_backup(state: &mut State, tag: &str) -> anyhow::Result<()> {
        let timestamp = Utc::now();
        let parsers_dir = PATHS.ts_parsers.join("parsers");
        let backup_dir = PATHS.ts_parsers.join("backups");
        let state_bak = PATHS.ts_parsers.join("state-parsers.json");
        let archive_path = backup_dir.join(format!(
            "backup-[{}]-[{tag}].zip",
            timestamp.format("%F_%H-%M-%S"),
        ));

        if !backup_dir.exists() {
            std::fs::create_dir_all(&backup_dir)?;
        }

        if !parsers_dir.exists() || parsers_dir.read_dir()?.next().is_none() {
            c_println!(amber, "WARNING: No parsers to backup");
            return Ok(());
        }

        let mut archive = File::create(&archive_path)?;
        let mut zip = ZipWriter::new(&mut archive);
        let options = FileOptions::default().compression_method(CompressionMethod::Bzip2);

        state.create_backup(&state_bak)?;
        zip_files(&mut zip, &options.unix_permissions(0o644), &state_bak)?;
        zip_files(&mut zip, &options.unix_permissions(0o755), &parsers_dir)?;
        zip.finish()?;

        state.append_restore_point(RestorePoint {
            date: timestamp,
            location: archive_path,
        });

        std::fs::remove_file(&state_bak)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{parsers::ParserInfo, state::ParserInstallMethod};
    use crate::utils::fs::remove_all;

    use super::*;

    fn dummy_parser_info() -> ParserInfo {
        ParserInfo {
            url: "https://test.com".to_owned(),
            files: vec!["src/test.c".to_owned()],
            location: None,
            revision: "test".to_owned(),
            generate_from_grammar: false,
        }
    }

    #[test]
    fn test_create_backup() {
        let mut state = State::new().unwrap();
        let parser_info = dummy_parser_info();
        state.add_parser("Test", "test", ParserInstallMethod::Compile, &parser_info);

        // check if backup was created
        let cwd = Path::new("tests").join("outputs").join("unit-tests");
        let backup_dir = cwd.join("backups");
        let parsers_dir = cwd.join("parsers");

        if backup_dir.exists() {
            remove_all(&backup_dir).unwrap();
        }

        if parsers_dir.exists() {
            remove_all(&parsers_dir).unwrap();
        }
        std::fs::create_dir_all(&parsers_dir).unwrap();
        std::fs::write(parsers_dir.join("test.txt"), "test").unwrap();

        Backups::create_backup(&mut state, "test").unwrap();

        for entry in backup_dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                assert_eq!(path.extension().unwrap(), "zip");
                assert!(path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("[test]"));
            }
        }
    }
}
