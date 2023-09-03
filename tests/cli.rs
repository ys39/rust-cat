use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn invalid_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("mycat")?;

    cmd.arg("-a");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-a' found",
    ));
    Ok(())
}

#[test]
fn help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("mycat")?;

    cmd.arg("--help");
    let contains_predicate =
        predicate::str::contains("  -A, --show-all          equivalent to -vET");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}
