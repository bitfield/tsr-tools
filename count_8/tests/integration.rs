use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["tests/data/test.txt", "tests/data/t2.txt"])
        .assert()
        .success()
        .stdout("tests/data/test.txt: 2\ntests/data/t2.txt: 3\n");
}

#[test]
fn binary_with_w_flag_counts_words() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["-w", "tests/data/test.txt"])
        .assert()
        .success()
        .stdout("tests/data/test.txt: 4\n");
}
