use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{} >> error.", bad);
    //let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str,
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-e", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> TestResult {
    run(&["-e", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> TestResult {
    run(&["-e", FOX], "tests/expected/fox.txt.b.out")
}

// --------------------------------------------------
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> TestResult {
    run(&["--empty-linenums", SPIDERS], "tests/expected/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> TestResult {
    run(
        &["--nonempty-linenums", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

// --------------------------------------------------
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

// --------------------------------------------------
fn bustle_b() -> TestResult {
    run(&["-e", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// --------------------------------------------------
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
fn all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-e"], "tests/expected/all.b.out")
}
