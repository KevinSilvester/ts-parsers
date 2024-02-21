use std::path::Path;

use jsonschema::is_valid;
use serde_json::Value;

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
