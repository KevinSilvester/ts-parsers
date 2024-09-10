mod utils;

mod no_updates {
    use assert_cmd::Command;

    use crate::utils::{setup, validate_state, OUTPUTS};

    #[test]
    fn test_update_specific() {
        let dir = OUTPUTS.join("test-update-no-updates-specific");
        setup(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["install", "rust", "blueprint", "markdown"]);
        cmd.assert().success();

        let expected = [
            r#"Parsers are already up-to-date: ["rust", "blueprint", "markdown"]"#,
            r#"No parsers to update!"#,
            r#"Gracefully shutting down... \(￣︶￣*\))"#,
            "",
        ];

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["update", "rust", "blueprint", "markdown"]);

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
        let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
        let status = output.status;

        assert!(status.success());
        assert_eq!(stdout, expected.join("\n"));
        assert_eq!(stderr, "");
        validate_state(&dir);
    }

    #[test]
    fn test_update_not_installed() {
        let dir = OUTPUTS.join("test-update-no-updates-not-installed");
        setup(&dir);
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["install", "rust"]);
        cmd.assert().success();

        let expected = [
            r#"Parsers are not installed: ["lua"]"#,
            r#"Parsers are already up-to-date: ["rust"]"#,
            r#"No parsers to update!"#,
            r#"Gracefully shutting down... \(￣︶￣*\))"#,
            "",
        ];
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["update", "rust", "lua"]);

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
        let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
        let status = output.status;

        assert!(status.success());
        assert_eq!(stdout, expected.join("\n"));
        assert_eq!(stderr, "");
        validate_state(&dir);
    }
}

mod do_updates {
    use assert_cmd::Command;

    use crate::utils::{check_backups, get_tag_by_index, setup, validate_state, OUTPUTS};

    #[test]
    fn test_update_specific() {
        let dir = OUTPUTS.join("test-update-do-updates-specific");
        setup(&dir);

        let old_tag = get_tag_by_index(1);
        let latest_tag = get_tag_by_index(0);

        dbg!(&old_tag, &latest_tag);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args([
            "install",
            "--tag",
            &old_tag,
            "rust",
            "markdown",
            "blueprint",
        ]);
        cmd.assert().success();
        validate_state(&dir);

        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.args(["update", "rust", "markdown"]);
        cmd.assert().success();
        validate_state(&dir);

        check_backups(&dir, 1, &latest_tag);
    }

    #[test]
    fn test_updated_locked_parsers() {
        let dir = OUTPUTS.join("test-update-locked-parsers");
        setup(&dir);

        let expected = [
            r#"Parsers are locked: ["lua", "blueprint", "markdown"]"#,
            r#"No parsers to update!"#,
            r#"Gracefully shutting down... \(￣︶￣*\))"#,
            "",
        ];

        let old_tag = get_tag_by_index(1);
        let latest_tag = get_tag_by_index(0);

        dbg!(&old_tag, &latest_tag);

        // install parsers
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["install", "--wanted", "--tag", &old_tag]);
        cmd.assert().success();
        validate_state(&dir);

        // lock parsers
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["lock", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        // uninstall parsers
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["update", "--wanted"]);

        let output = cmd.output().unwrap();
        let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
        let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
        let status = output.status;

        assert!(status.success());
        assert_eq!(stdout, expected.join("\n"));
        assert_eq!(stderr, "");
        validate_state(&dir);
        check_backups(&dir, 0, "");

        // unlock parsers
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["unlock", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);

        // uninstall parsers
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        cmd.env("TS_PARSERS_DATA", &dir);
        cmd.env(
            "TS_PARSERS_WANTED_PARSERS",
            "tests/fixtures/wanted-parsers.txt",
        );
        cmd.args(["update", "--wanted"]);
        cmd.assert().success();
        validate_state(&dir);
        check_backups(&dir, 1, &latest_tag);

        let parser_dir = dir.join("parser");
        let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
        assert_eq!(files.len(), 3);
    }
}
