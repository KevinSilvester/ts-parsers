#![allow(dead_code)]

use std::path::Path;

use jsonschema::is_valid;
use serde_json::Value;

pub const WANT_PARSERS: &[&str] = &["lua", "blueprint", "markdown"];

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
    std::fs::write(test_dir.join("wanted-parsers.txt"), WANT_PARSERS.join("\n")).unwrap();
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
