
use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use std::fs;

#[test]
fn yeet() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gSV")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("example_data/test3")
        .arg("--nestedness");

    cmd.assert().success();
    //fs::remove_file("example_data/test3.bubble.stats")?;
    //fs::remove_file("example_data/test3.bubble.txt")?;
    //fs::remove_file("example_data/test3.traversal.bed")?;


    Ok(())
}

#[test]
fn bifurcation_simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gSV")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg_local/Rust/gSV/example_data/testGraph.gfa")
        .arg("-o")
        .arg("example_data/test3")
        .arg("--nestedness")
        .arg("-b");

    cmd.assert().success();
    fs::remove_file("example_data/test3.bubble.txt")?;
    //fs::remove_file("example_data/test3.bubble.txt")?;
    //fs::remove_file("example_data/test3.traversal.bed")?;


    Ok(())
}