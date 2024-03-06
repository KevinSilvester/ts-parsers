mod utils;

use assert_cmd::Command;
use utils::{check_backups, get_tag_by_index, OUTPUTS};

use crate::utils::{setup, validate_state};

#[test]
fn test_install_specific() {
    let dir = OUTPUTS.join("test-install-specific");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["install", "rust", "blueprint", "markdown"]);
    cmd.assert().success();

    validate_state(&dir);
}

#[test]
fn test_install_wanted() {
    let dir = OUTPUTS.join("test-install-wanted");
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
}

#[test]
fn test_install_with_zig() {
    let dir = OUTPUTS.join("test-install-with-zig");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["install", "--compiler", "zig", "rust"]);
    cmd.assert().success();

    validate_state(&dir);
}

#[test]
fn test_install_force() {
    let tag = get_tag_by_index(0);
    let dir = OUTPUTS.join("test-install-force");

    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["install", "lua"]);
    cmd.assert().success();

    validate_state(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_DATA", &dir);
    cmd.args(["install", "--force", "lua"]);
    cmd.assert().success();

    validate_state(&dir);
    check_backups(&dir, 1, &tag);
}
