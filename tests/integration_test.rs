use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("add").arg("-c").arg("Test").arg("-d").arg("today");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
fn test_done() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("done").arg("-i").arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
fn test_remove() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("remove").arg("-i").arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ok!"));

    Ok(())
}

#[test]
fn test_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jourst")?;

    cmd.arg("list").arg("-k").arg("all");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("id"));

    Ok(())
}
