mod utils;

mod unlock_specific {
    use assert_cmd::Command;

    use crate::utils::{setup, validate_state, OUTPUTS};

    #[test]
    fn test_installed() {
        let dir = OUTPUTS.join("test-unlock-specific-installed");
        setup(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["install", "rust", "blueprint", "markdown"]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["lock", "rust", "blueprint"]);
        cmd.assert().success();
        validate_state(&dir);

        let parser_dir = dir.join("parser");
        assert!(parser_dir.is_dir());

        let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
        assert_eq!(files.len(), 3);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(state["parsers"]["rust"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["markdown"]["locked"].as_bool().unwrap());

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["unlock", "rust"]);
        cmd.assert().success();
        validate_state(&dir);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(!state["parsers"]["rust"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["markdown"]["locked"].as_bool().unwrap());
    }

    #[test]
    fn test_not_installed() {
        let dir = OUTPUTS.join("test-unlock-specific-not-installed");
        setup(&dir);

        let expected = [
            r#"[ERROR]: Parsers are not installed: ["lua"]"#,
            r#"I borked... (┬┬﹏┬┬)"#,
            "",
        ];

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["install", "rust", "blueprint", "markdown"]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["lock", "rust", "blueprint"]);
        cmd.assert().success();
        validate_state(&dir);

        let parser_dir = dir.join("parser");
        assert!(parser_dir.is_dir());

        let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
        assert_eq!(files.len(), 3);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(state["parsers"]["rust"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["markdown"]["locked"].as_bool().unwrap());

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["unlock", "rust", "lua", "blueprint"]);

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
        let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
        let status = output.status;

        assert!(!status.success());
        assert_eq!(stdout, "");
        assert_eq!(stderr, expected.join("\n"));

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(state["parsers"]["rust"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["markdown"]["locked"].as_bool().unwrap());
    }
}

mod unlock_wanted {
    use assert_cmd::Command;

    use crate::utils::{setup, validate_state, OUTPUTS};

    #[test]
    fn test_installed() {
        let dir = OUTPUTS.join("test-unlock-wanted-installed");
        setup(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["install", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["lock", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        let parser_dir = dir.join("parser");
        assert!(parser_dir.is_dir());

        let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
        assert_eq!(files.len(), 3);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(state["parsers"]["lua"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["markdown"]["locked"].as_bool().unwrap());

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["unlock", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(!state["parsers"]["lua"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(!state["parsers"]["markdown"]["locked"].as_bool().unwrap());
    }

    #[test]
    fn test_not_installed() {
        let dir = OUTPUTS.join("test-unlock-wanted-not-installed");
        setup(&dir);

        let expected = [
            r#"[ERROR]: Parsers are not installed: ["rust"]"#,
            r#"I borked... (┬┬﹏┬┬)"#,
            "",
        ];

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["install", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["lock", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["unlock", "rust", "lua", "blueprint"]);

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
        let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
        let status = output.status;

        assert!(!status.success());
        assert_eq!(stdout, "");
        assert_eq!(stderr, expected.join("\n"));

        let parser_dir = dir.join("parser");
        assert!(parser_dir.is_dir());

        let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
        assert_eq!(files.len(), 3);

        let state_file = dir.join("state.json");
        let state_str = std::fs::read_to_string(&state_file).unwrap();
        let state: serde_json::Value = serde_json::from_str(&state_str).unwrap();

        assert!(state["parsers"]["lua"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["blueprint"]["locked"].as_bool().unwrap());
        assert!(state["parsers"]["markdown"]["locked"].as_bool().unwrap());
    }
}
