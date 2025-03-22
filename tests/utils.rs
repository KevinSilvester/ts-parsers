#![allow(dead_code)]

use std::path::{Path, PathBuf};

use jsonschema::is_valid;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    pub static ref OUTPUTS: PathBuf = PathBuf::from("tests").join("outputs");
    pub static ref WANTED_PARSERS: Vec<String> = {
        let wanted_parsers = std::fs::read_to_string("tests/fixtures/wanted-parsers.txt").unwrap();
        let res = wanted_parsers
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        res
    };
}

pub fn remove_all(path: &Path) {
    if path.is_file() {
        std::fs::remove_file(path).unwrap();
        return;
    }

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            remove_all(&path);
        } else {
            std::fs::remove_file(&path).unwrap();
        }
    }

    std::fs::remove_dir(path).unwrap();
}

pub fn setup(test_dir: &Path) {
    if test_dir.exists() {
        remove_all(test_dir)
    }

    std::fs::create_dir_all(test_dir).unwrap();
}

pub fn validate_state(dir: &Path) {
    let state_path = dir.join("state.json");
    let schema_path = Path::new("tests")
        .join("fixtures")
        .join("state-schema.json");
    let state: Value = serde_json::from_str(&std::fs::read_to_string(state_path).unwrap()).unwrap();
    let schema: Value =
        serde_json::from_str(&std::fs::read_to_string(schema_path).unwrap()).unwrap();

    assert!(is_valid(&schema, &state));
}

pub fn get_tag_by_index(index: usize) -> String {
    #[cfg(unix)]
    let shell = "bash";

    #[cfg(windows)]
    let shell = "pwsh";

    let script = format!(
        "curl -s 'https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/master/CHANGELOG.json' | jq -r '.[{index}].tag'"
    );

    let output = std::process::Command::new(shell)
        .args(["-c", &script])
        .output()
        .unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_owned()
}

pub fn check_backups(dir: &Path, num: i32, tag: &str) {
    let backups = dir.join("backups");
    let mut count = 0;

    if !backups.exists() {
        return;
    }

    if num == 0 {
        assert!(backups.read_dir().unwrap().next().is_none());
        return;
    }

    for entry in backups.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        dbg!(&name);

        assert!(path.is_file());
        assert!(name.contains(tag));
        assert!(name.ends_with(".bz2"));

        count += 1;
    }

    assert_eq!(count, num);
}
