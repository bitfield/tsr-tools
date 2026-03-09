use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    cargo_bin_cmd!("count_8")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    cargo_bin_cmd!("count_8")
        .args(["tests/data/test.txt", "tests/data/t2.txt"])
        .assert()
        .success()
        .stdout("tests/data/test.txt: 2 lines\ntests/data/t2.txt: 3 lines\n");
}

#[test]
fn binary_with_w_flag_counts_words() {
    cargo_bin_cmd!("count_8")
        .args(["-w", "tests/data/test.txt"])
        .assert()
        .success()
        .stdout("tests/data/test.txt: 4 words\n");
}
