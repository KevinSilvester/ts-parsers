mod common;
mod utils;

use std::path::PathBuf;

use assert_cmd::Command;
use common::validate_state;
use lazy_static::lazy_static;

use crate::utils::setup;

lazy_static! {
    static ref OUTPUTS: PathBuf = PathBuf::from("tests").join("outputs");
}

#[test]
fn test_install_specific() {
    let dir = OUTPUTS.join("test-install-specific");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args(["install", "rust", "blueprint", "markdown"]);
    cmd.assert().success();

    validate_state(&dir);
}

#[test]
fn test_install_wanted() {
    let dir = OUTPUTS.join("test-install-wanted");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args(["install", "--wanted"]);
    cmd.assert().success();

    validate_state(&dir);
}

#[test]
fn test_install_with_zig() {
    let dir = OUTPUTS.join("test-install-with-zig");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args(["install", "--compiler", "zig", "rust"]);
    cmd.assert().success();

    validate_state(&dir);
}

#[test]
fn test_install_force() {
    let dir = OUTPUTS.join("test-install-force");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args(["install", "lua"]);
    cmd.assert().success();

    validate_state(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args(["install", "--force", "lua"]);
    cmd.assert().success();

    validate_state(&dir);
}
