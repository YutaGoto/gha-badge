use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn cli_with_link() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gha-badge")?;
    cmd.arg("-n").arg("sample").arg("--with-link");
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("[![action]"));

    Ok(())
}

#[test]
fn cli_without_link() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gha-badge")?;
    cmd.arg("-n").arg("sample");
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("![action]"));

    Ok(())
}

#[test]
fn not_found_github_directory() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gha-badge")?;
    cmd.current_dir("/tmp");
    cmd.arg("-n").arg("sample");
    cmd.assert().failure();
    cmd.assert()
        .stderr(predicate::str::contains("Not found .github/workflows"));

    Ok(())
}

#[test]
fn not_set_github_username() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gha-badge")?;
    cmd.assert().failure();
    cmd.assert()
        .stderr(predicate::str::contains("Not set GITHUB_USERNAME"));

    Ok(())
}
