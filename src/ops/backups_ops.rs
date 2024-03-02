use chrono::Utc;

use crate::{
    c_println,
    data::state::{RestorePoint, State},
    utils::{archives, fs as ufs, PATHS},
};

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

pub fn restore_backup(state: &mut State, id: usize) -> anyhow::Result<()> {
    let restore_point = state.get_restore_point(id)?;
    let tmp_dir = tempfile::tempdir()?;

    archives::extract_tar_bz2(&restore_point.location, tmp_dir.path())?;
    ufs::remove_all(PATHS.ts_parsers.join("parsers"))?;
    std::fs::create_dir_all(PATHS.ts_parsers.join("parsers"))?;

    ufs::copy_all(
        tmp_dir.path().join("parsers"),
        PATHS.ts_parsers.join("parsers"),
    )?;

    std::fs::remove_file(&restore_point.location)?;
    state.restore_backup(tmp_dir.path().join("state-parsers.json"))?;
    state.delete_restore_point(id);

    Ok(())
}

pub fn delete_backup(state: &mut State, ids: Vec<usize>, all: bool) -> anyhow::Result<()> {
    if all {
        state.delete_all_restore_points();
    } else {
        for id in ids {
            state.delete_restore_point(id);
        }
    }
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

    fn clear_dir(dir: &Path) {
        if dir.exists() {
            remove_all(dir).unwrap();
        }
    }

    #[test]
    fn test_create_and_restore_backup() {
        let cwd = Path::new("tests").join("outputs").join("unit-tests");
        let backup_dir = cwd.join("backups");
        let parsers_dir = cwd.join("parsers");
        let state_file = cwd.join("state.json");

        if state_file.exists() {
            std::fs::remove_file(&state_file).unwrap();
        }

        let mut state = State::new().unwrap();
        let parser_info = dummy_parser_info();

        clear_dir(&backup_dir);
        clear_dir(&parsers_dir);
        std::fs::create_dir_all(&parsers_dir).unwrap();

        // Add a parser to the state
        state.add_parser(
            "test-parser-1",
            "test-tag",
            ParserInstallMethod::Compile,
            &parser_info,
        );
        std::fs::write(parsers_dir.join("test-parser-1"), "").unwrap();

        // create a backup
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

        // Add another parser to the state
        state.add_parser(
            "test-parser-2",
            "test-tag",
            ParserInstallMethod::Compile,
            &parser_info,
        );
        std::fs::write(parsers_dir.join("test-parser-2"), "").unwrap();

        // commit the state
        state.commit().unwrap();

        // restore the backup
        assert!(state.check_restore_exists(0).unwrap());
        restore_backup(&mut state, 0).unwrap();

        // check if the state was restored
        assert_eq!(state.parsers.len(), 1);
        assert!(state.parsers.get("test-parser-1").is_some());
        assert!(state.parsers.get("test-parser-2").is_none());

        // check if restore point was deleted
        assert!(state.check_restore_exists(0).is_err());

        // check if the backup was deleted
        assert_eq!(backup_dir.read_dir().unwrap().count(), 0);

        // check if the parsers were restored
        for entry in parsers_dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                assert_eq!(path.file_name().unwrap(), "test-parser-1");
            }
        }
    }
}
