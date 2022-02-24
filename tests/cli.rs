use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use env_logger::Target; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gSV")?;
    cmd.arg("-g example_data/testGrapdsadh.gfa").arg("-o hello");
    cmd.assert().stderr(predicate::str::contains("No file with such name"));

    Ok(())
}

#[test]
fn file_doesnt_exist2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gSV")?;
    cmd.arg("-g example_data/testGraph.gfa").arg("-o jo");
    cmd.assert().success();
    Ok(())
}
