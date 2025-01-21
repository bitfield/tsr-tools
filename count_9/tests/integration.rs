use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin("count_9")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    let want = "tests/data/test.txt: 2 lines\ntests/data/test2.txt: 3 lines\n";
    Command::cargo_bin("count_9")
        .unwrap()
        .args(["tests/data/test.txt", "tests/data/test2.txt"])
        .assert()
        .success()
        .stdout(predicate::eq(want));
}

#[test]
fn binary_with_w_flag_counts_words() {
    let want = "tests/data/test.txt: 4 words\n";
    Command::cargo_bin("count_9")
        .unwrap()
        .args(["-w", "tests/data/test.txt"])
        .assert()
        .success()
        .stdout(predicate::eq(want));
}
