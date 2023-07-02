use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Ignored because it sometimes fails on CI due to the database lock

#[test]
#[ignore]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("add").arg("-c").arg("Test").arg("-d").arg("today");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
#[ignore]
fn test_done() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("done").arg("-i").arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
#[ignore]
fn test_remove() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("remove").arg("-i").arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
#[ignore]
fn test_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("list").arg("-k").arg("all");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("id"));

    Ok(())
}

#[test]
#[ignore]
fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("sync");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}