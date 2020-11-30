use assert_cmd::prelude::*;
use std::process::Command;

type Error = Box<dyn std::error::Error>;


#[test]
fn get_info() -> Result<(), Error> {
    Command::cargo_bin("trackme")?
        .arg("info")
        .assert()
        .success();

    Ok(())
}

#[test]
fn list_available_tracking_entries() -> Result<(), Error> {
    Command::cargo_bin("trackme")?
        .arg("list")
        .assert()
        .success();

    Ok(())
}

#[test]
fn show_proper_entry_types_structures() -> Result<(), Error> {
    Command::cargo_bin("trackme")?
        .arg("show")
        .arg("-e")
        .arg("act")
        .arg("-u")
        .arg("info")
        .assert()
        .success();

    Ok(())
}

#[test]
fn show_proper_entry_types_example() -> Result<(), Error> {
    // short options
    Command::cargo_bin("trackme")?
        .arg("show")
        .arg("-e")
        .arg("act")
        .arg("-u")
        .arg("example")
        .assert()
        .success();

    // long options
    Command::cargo_bin("trackme")?
        .arg("show")
        .arg("--entrytype")
        .arg("med")
        .arg("--usage")
        .arg("info")
        .assert()
        .success();

    Ok(())
}

#[test]
fn improper_save_command() -> Result<(), Error> {
    Command::cargo_bin("trackme")?
        .arg("save")
        .arg("-e")
        .arg("lol")
        .arg("-d")
        .arg("lollosd,sdsafd")
        .arg("-f")
        .arg("test.csv")
        .assert()
        .failure();
    
    Ok(())
}

#[test]
fn improper_data_structure_to_save() -> Result<(), Error> {
    Command::cargo_bin("trackme")?
        .arg("save")
        .arg("-e")
        .arg("act")
        .arg("-d")
        .arg("lol,rofl,lololo")
        .arg("-f")
        .arg("test.csv")
        .assert()
        .failure();

    Ok(())
}
