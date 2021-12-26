use assert_cmd::Command;
use std::error::Error;
use std::fs;

const PRG: &str = "rcat";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_empty_file() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.arg(EMPTY).assert().success().stdout("");

    Ok(())
}

#[test]
fn test_one_line_file() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    let file = read_file_with_new_line(FOX);

    cmd.arg(FOX).assert().success().stdout(file);

    Ok(())
}

fn read_file_with_new_line(filename: &str) -> String {
    let mut file = fs::read_to_string(filename).unwrap();
    file.push_str("\n");

    file
}
