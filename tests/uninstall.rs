mod utils;

use assert_cmd::Command;
use utils::OUTPUTS;

use crate::utils::{setup, validate_state};

#[test]
fn test_uninstall_specific() {
    let dir = OUTPUTS.join("test-uninstall-specific");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["install", "rust", "blueprint", "markdown"]);
    cmd.assert().success();
    validate_state(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["uninstall", "rust", "blueprint"]);
    cmd.assert().success();
    validate_state(&dir);

    let parser_dir = dir.join("parsers");
    assert!(parser_dir.is_dir());

    let backup_dir = dir.join("backups");
    assert!(backup_dir.is_dir());

    let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
    assert_eq!(files.len(), 1);

    let file = files[0].as_ref().unwrap();
    assert_eq!(file.file_name(), "markdown.so");
}

#[test]
fn test_uninstall_wanted() {
    let dir = OUTPUTS.join("test-uninstall-wanted");
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
    cmd.args(["uninstall", "--wanted"]);
    cmd.assert().success();
    validate_state(&dir);

    let parser_dir = dir.join("parsers");
    assert!(parser_dir.is_dir());

    let backup_dir = dir.join("backups");
    assert!(backup_dir.is_dir());

    let files = parser_dir.read_dir().unwrap().collect::<Vec<_>>();
    assert_eq!(files.len(), 0);
}

#[test]
fn test_uninstall_not_installed() {
    let dir = OUTPUTS.join("test-uninstall-not-installed");
    setup(&dir);

    let expected = [
        r#"Error: Parsers are not installed: ["lua"]"#,
        r#"I borked... (┬┬﹏┬┬)"#,
        "",
    ];
    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["uninstall", "lua"]);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(strip_ansi_escapes::strip(output.stdout)).unwrap();
    let stderr = String::from_utf8(strip_ansi_escapes::strip(output.stderr)).unwrap();
    let status = output.status;

    assert!(!status.success());
    assert_eq!(stdout, "");
    assert_eq!(stderr, expected.join("\n"));
}
