mod utils;

use std::path::PathBuf;

use assert_cmd::Command;
use lazy_static::lazy_static;
use utils::WANT_PARSERS;

use crate::utils::setup;

lazy_static! {
    static ref OUTPUTS: PathBuf = PathBuf::from("tests").join("outputs");
}

#[test]
fn test_compile_specific() {
    let dir = OUTPUTS.join("test-compile-specific");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.args([
        "compile",
        "--destination",
        &dir.to_str().unwrap(),
        "rust",
        "blueprint",
        "markdown",
    ]);
    cmd.assert().success();

    for lang in ["rust", "blueprint", "markdown"] {
        assert!(dir.join(format!("{lang}.so")).exists());
    }
}

#[test]
fn test_compile_wanted() {
    let dir = OUTPUTS.join("test-compile-wanted");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.env("TS_PARSERS_TEST_DIR", &dir);
    cmd.args([
        "compile",
        "--destination",
        dir.to_str().unwrap(),
        "--wanted",
    ]);
    cmd.assert().success();

    for lang in WANT_PARSERS {
        assert!(dir.join(format!("{lang}.so")).exists());
    }
}

#[test]
fn test_compile_with_zig() {
    let dir = OUTPUTS.join("test-compile-with-zig");
    let zig_targets = [
        "x86_64-linux",
        "aarch64-linux",
        "x86_64-macos",
        "aarch64-macos",
        "x86_64-windows",
    ];
    setup(&dir);

    for target in &zig_targets {
        let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
        let destination = dir.join(target);

        cmd.args([
            "compile",
            "--destination",
            destination.to_str().unwrap(),
            "--compiler",
            "zig",
            "--target",
            target,
            "lua",
        ]);
        cmd.assert().success();

        assert!(destination.join("lua.so").exists());
    }
}
