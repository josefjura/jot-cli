use std::{fs::create_dir, path::Path};

use assert_cmd::Command;
use predicates::prelude::{
    predicate::str::{contains, is_empty},
    PredicateBooleanExt,
};
use tempfile::TempDir;

#[test]
fn test_profile_arg() {
    let mut cmd = Command::cargo_bin("jot-cli").unwrap();

    let assert = cmd
        .env("JOT_PROFILE", "bad_test.toml")
        .args(&["--profile", "test_assets/profile/default.toml"])
        .arg("config")
        .assert();

    assert
        .success()
        .stdout(
            contains(r#""profile_path": "test_assets/profile/default.toml""#)
                .and(contains(r#""server_url": "asset_toml_server_url""#)),
        )
        .stderr(is_empty());
}

#[test]
fn test_profile_env() {
    let mut cmd = Command::cargo_bin("jot-cli").unwrap();

    let assert = cmd
        .env("JOT_PROFILE", "test_assets/profile/default.toml")
        .arg("config")
        .assert();

    assert
        .success()
        .stdout(
            contains(r#""profile_path": "test_assets/profile/default.toml""#)
                .and(contains(r#""server_url": "asset_toml_server_url""#)),
        )
        .stderr(is_empty());
}

#[test]
fn test_login() {
    let mut cmd = Command::cargo_bin("jot-cli").unwrap();
    let dir = TempDir::new().unwrap();
    let dir_path = dir.path();
    let file_path = dir_path.join(Path::new("local.toml"));
    std::fs::copy("test_assets/profile/local.toml", &file_path).unwrap();

    let assert = cmd
        .env("JOT_PROFILE", file_path.to_str().unwrap())
        .arg("-m")
        // .args(&["--mock-param", "test_assets/profile/default.toml"])
        .arg("login")
        .assert();

    assert
        .success()
        .stdout(
            contains(r#"Mocking sending device code"#)
                .and(contains(r#"Please visit this URL to login"#))
                .and(contains(r#"Mocking polling for token with device code:"#).count(3)),
        )
        .stderr(is_empty());
}
