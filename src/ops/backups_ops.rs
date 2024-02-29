use chrono::Utc;

use crate::{utils::{PATHS, archives}, c_println, data::state::{State, RestorePoint}};

pub fn create_backup(state: &mut State, tag: &str) -> anyhow::Result<()> {
    let timestamp = Utc::now();
    let parsers_dir = PATHS.ts_parsers.join("parsers");
    let backup_dir = PATHS.ts_parsers.join("backups");
    let state_bak = PATHS.ts_parsers.join("state-parsers.json");
    let archive_path = backup_dir.join(format!(
        "backup-[{}]-[{tag}].tar.bz2",
        timestamp.format("%F_%H-%M-%S"),
    ));

    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)?;
    }

    if !parsers_dir.exists() || parsers_dir.read_dir()?.next().is_none() {
        c_println!(blue, "INFO: No parsers to backup");
        return Ok(());
    }


    state.create_backup(&state_bak)?;
    archives::create_tar_bz2(&archive_path, &[&parsers_dir, &state_bak])?;

    state.append_restore_point(RestorePoint {
        date: timestamp,
        location: archive_path,
    });

    std::fs::remove_file(&state_bak)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::path::Path;

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

        create_backup(&mut state, "test").unwrap();

        for entry in backup_dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                assert_eq!(path.extension().unwrap(), "bz2");
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
