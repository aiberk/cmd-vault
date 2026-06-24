use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

// Integration tests for CLI functionality
#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A blazingly fast terminal command manager"));
}

#[test]
fn test_list_empty() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("test-config.json");

    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.env("CMD_VAULT_CONFIG", config_path)
        .arg("-l")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 commands"));
}

#[test]
fn test_add_command() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("test-config.json");

    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.env("CMD_VAULT_CONFIG", config_path.clone())
        .args(&["-a", "test-cmd", "echo hello", "A test command"])
        .assert()
        .success();

    // Verify the command was added
    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("test-cmd"));
    assert!(contents.contains("echo hello"));
}

#[test]
fn test_search_command() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("test-config.json");

    // First add a command
    Command::cargo_bin("cmd-vault").unwrap()
        .env("CMD_VAULT_CONFIG", config_path.clone())
        .args(&["-a", "docker-build", "docker build -t myapp .", "Build Docker image"])
        .assert()
        .success();

    // Then search for it
    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.env("CMD_VAULT_CONFIG", config_path)
        .args(&["-s", "docker"])
        .assert()
        .success()
        .stdout(predicate::str::contains("docker-build"));
}