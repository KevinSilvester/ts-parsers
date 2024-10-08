mod utils;

use assert_cmd::Command;
use utils::{OUTPUTS, WANTED_PARSERS};

use crate::utils::setup;

#[test]
fn test_compile_specific() {
    let dir = OUTPUTS.join("test-compile-specific");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.args([
        "compile",
        "--destination",
        dir.to_str().unwrap(),
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
    cmd.env(
        "TS_PARSERS_WANTED_PARSERS",
        "tests/fixtures/wanted-parsers.txt",
    );
    cmd.args([
        "compile",
        "--destination",
        dir.to_str().unwrap(),
        "--wanted",
    ]);
    cmd.assert().success();

    for lang in WANTED_PARSERS.iter() {
        assert!(dir.join(format!("{lang}.so")).exists());
    }
}

#[test]
fn test_compile_with_gcc() {
    let dir = OUTPUTS.join("test-compile-with-gcc");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.args([
        "compile",
        "--compiler",
        "gcc",
        "--destination",
        dir.to_str().unwrap(),
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

#[test]
fn test_from_grammar_compile() {
    let dir = OUTPUTS.join("test-from-grammar-and-compile");
    setup(&dir);

    let mut cmd = Command::cargo_bin("ts-parsers").unwrap();
    cmd.args([
        "compile",
        "--destination",
        dir.to_str().unwrap(),
        "--from-grammar",
        "--npm",
        "pnpm",
        "rust",
        "blueprint",
        "markdown",
    ]);
    cmd.assert().success();

    for lang in ["rust", "blueprint", "markdown"] {
        assert!(dir.join(format!("{lang}.so")).exists());
    }
}
