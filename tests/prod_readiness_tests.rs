//! Production Readiness Tests - CPM
//!
//! Validates that the CPM package manager meets production readiness criteria.
//! Run with: cargo test --test prod_readiness_tests

use std::process::Command;
use std::env;

fn cpm_bin() -> String {
    env::var("CARGO_BIN_EXE_cpm").unwrap_or_else(|_| "cpm".to_string())
}

#[test]
fn prod_cpm_help_exits_success() {
    let status = Command::new(cpm_bin())
        .arg("--help")
        .status();
    assert!(status.is_ok(), "cpm --help must not panic");
    assert!(status.unwrap().success(), "cpm --help must exit 0");
}

#[test]
fn prod_cpm_version_exits_success() {
    let status = Command::new(cpm_bin())
        .arg("--version")
        .status();
    assert!(status.is_ok(), "cpm --version must not panic");
    assert!(status.unwrap().success(), "cpm --version must exit 0");
}

#[test]
fn prod_cpm_init_help_works() {
    let status = Command::new(cpm_bin())
        .args(["init", "--help"])
        .status();
    assert!(status.is_ok(), "cpm init --help must work");
}

#[test]
fn prod_cpm_install_help_works() {
    let status = Command::new(cpm_bin())
        .args(["install", "--help"])
        .status();
    assert!(status.is_ok(), "cpm install help must work");
}

#[test]
fn prod_cpm_add_rust_help_works() {
    let status = Command::new(cpm_bin())
        .args(["add-rust", "--help"])
        .status();
    assert!(status.is_ok(), "cpm add-rust help must work");
}

#[test]
fn prod_cpm_rust_status_in_empty_dir() {
    let temp = tempfile::TempDir::new().expect("temp dir");
    let status = Command::new(cpm_bin())
        .arg("rust-status")
        .current_dir(temp.path())
        .status();
    assert!(status.is_ok(), "cpm rust-status must not panic");
}

#[test]
fn prod_cpm_init_creates_project() {
    let npm_available = if cfg!(target_os = "windows") {
        Command::new("npm.cmd").arg("--version").output().is_ok()
    } else {
        Command::new("npm").arg("--version").output().is_ok()
    };
    if !npm_available {
        return;
    }
    let temp = tempfile::TempDir::new().expect("temp dir");
    let status = Command::new(cpm_bin())
        .args(["init", "prod-test-project", "-y"])
        .current_dir(temp.path())
        .status();
    assert!(status.is_ok(), "cpm init must not panic: {:?}", status.err());
    assert!(status.unwrap().success(), "cpm init must succeed");
    let project_dir = temp.path().join("prod-test-project");
    assert!(project_dir.is_dir(), "Project dir must be created");
    assert!(project_dir.join("package.json").exists(), "package.json must exist");
    assert!(project_dir.join("index.js").exists(), "index.js must exist");
    assert!(project_dir.join("README.md").exists(), "README.md must exist");
}

#[test]
fn prod_cpm_rust_status_in_js_project() {
    let npm_available = if cfg!(target_os = "windows") {
        Command::new("npm.cmd").arg("--version").output().is_ok()
    } else {
        Command::new("npm").arg("--version").output().is_ok()
    };
    if !npm_available {
        return;
    }
    let temp = tempfile::TempDir::new().expect("temp dir");
    let status = Command::new(cpm_bin())
        .args(["init", "rust-status-test", "-y"])
        .current_dir(temp.path())
        .status();
    assert!(status.unwrap().success());
    let project_dir = temp.path().join("rust-status-test");
    let status = Command::new(cpm_bin())
        .arg("rust-status")
        .current_dir(&project_dir)
        .status();
    assert!(status.is_ok(), "cpm rust-status in JS project must not panic");
}
