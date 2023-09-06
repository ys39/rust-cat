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
        predicate::str::contains("  -A, --show-all          equivalent to -vET")
            .and(predicate::str::contains(
                "  -b, --number-nonblank   number nonempty output lines, overrides -n",
            ))
            .and(predicate::str::contains(
                "  -e                      equivalent to -vE",
            ))
            .and(predicate::str::contains(
                "  -E, --show-ends         display $ at end of each line",
            ))
            .and(predicate::str::contains(
                "  -n, --number            number all output lines",
            ))
            .and(predicate::str::contains(
                "  -s, --squeeze-blank     suppress repeated empty output lines",
            ))
            .and(predicate::str::contains(
                "  -t                      equivalent to -vT",
            ))
            .and(predicate::str::contains(
                "  -T, --show-tabs         display TAB characters as ^I",
            ))
            .and(predicate::str::contains(
                "  -u                      (ignored)",
            ))
            .and(predicate::str::contains(
                "  -v, --show-nonprinting  use ^ and M- notation, except for LFD and TAB",
            ))
            .and(predicate::str::contains(
                "  -h, --help              Print help",
            ))
            .and(predicate::str::contains(
                "  -V, --version           Print version",
            ));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}
