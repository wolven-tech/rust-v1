use std::fs;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_meta_version() {
    let mut cmd = cargo_bin_cmd!("meta");
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("meta"));
}

#[test]
fn test_meta_help() {
    let mut cmd = cargo_bin_cmd!("meta");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Meta task orchestrator"));
}

#[test]
fn test_meta_init_creates_config() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = cargo_bin_cmd!("meta");

    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    // Verify meta.toml was created
    let config_path = temp_dir.path().join("meta.toml");
    assert!(config_path.exists());

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("[workspace]"));
    assert!(content.contains("[tools.turborepo]"));
    assert!(content.contains("[tools.bacon]"));
}

#[test]
fn test_meta_init_detects_rust_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create apps directory with a Rust project
    let api_path = temp_dir.path().join("apps/api");
    fs::create_dir_all(&api_path).unwrap();
    fs::write(
        api_path.join("Cargo.toml"),
        r#"[package]
name = "api"
version = "1.0.0"
"#,
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("meta");
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    let config_path = temp_dir.path().join("meta.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("[projects.api]"));
    assert!(content.contains("type = \"rust\""));
    assert!(content.contains("path = \"apps/api\""));
}

#[test]
fn test_meta_init_detects_next_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create apps directory with a Next.js project
    let web_path = temp_dir.path().join("apps/web");
    fs::create_dir_all(&web_path).unwrap();
    fs::write(
        web_path.join("package.json"),
        r#"{
  "name": "@test/web",
  "dependencies": {
    "next": "14.0.0",
    "react": "18.0.0"
  }
}"#,
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("meta");
    cmd.current_dir(&temp_dir);
    cmd.arg("init");
    cmd.assert().success();

    let config_path = temp_dir.path().join("meta.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("[projects.web]"));
    assert!(content.contains("type = \"next\""));
    assert!(content.contains("@test/web"));
}

#[test]
fn test_meta_dev_requires_config() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = cargo_bin_cmd!("meta");

    cmd.current_dir(&temp_dir);
    cmd.arg("dev");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("meta.toml not found"));
}

#[test]
fn test_meta_build_requires_config() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = cargo_bin_cmd!("meta");

    cmd.current_dir(&temp_dir);
    cmd.arg("build");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("meta.toml not found"));
}

#[test]
fn test_meta_test_requires_config() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = cargo_bin_cmd!("meta");

    cmd.current_dir(&temp_dir);
    cmd.arg("test");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("meta.toml not found"));
}
