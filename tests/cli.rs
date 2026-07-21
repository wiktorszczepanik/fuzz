use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::str::contains;
use std::fs::write;
use tempfile::NamedTempFile;

#[test]
fn cli_should_find_matching_lines() {
    let file = NamedTempFile::new().unwrap();
    write(file.path(), "Lorem ipsum\ndolor sit amet\nLorem else\n").unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lorem", file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Lorem ipsum").and(contains("Lorem else")));
}

#[test]
fn cli_should_find_matching_lines_with_typo() {
    let file = NamedTempFile::new().unwrap();
    write(
        file.path(),
        "Lorem ipsum\ndolor sit amet\nLorem else\nNext line...",
    )
    .unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lroem", file.path().to_str().unwrap()]) // typo
        .assert()
        .success()
        .stdout(contains("Lorem ipsum").and(contains("Lorem else")));
}

#[test]
fn cli_should_support_fuzzy_search() {
    let file = NamedTempFile::new().unwrap();
    write(file.path(), "Lorem ipsum\nOne\ntwo").unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lroem", file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Lorem ipsum"));
}

#[test]
fn cli_should_show_line_numbers() {
    let file = NamedTempFile::new().unwrap();
    write(file.path(), "first line\nLorem ipsum\nthird line\n").unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lorem", file.path().to_str().unwrap(), "--lines"])
        .assert()
        .success()
        .stdout(contains("2: Lorem ipsum"));
}

#[test]
fn cli_should_show_scores() {
    let file = NamedTempFile::new().unwrap();
    write(file.path(), "Lorem ipsum\n").unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lorem", file.path().to_str().unwrap(), "--score"])
        .assert()
        .success()
        .stdout(contains("["));
}

#[test]
fn cli_should_show_line_numbers_and_scores() {
    let file = NamedTempFile::new().unwrap();
    write(file.path(), "Lorem ipsum\n").unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lorem", file.path().to_str().unwrap(), "--lines", "--score"])
        .assert()
        .success()
        .stdout(
            contains("1:")
                .and(contains("["))
                .and(contains("Lorem ipsum")),
        );
}

#[test]
fn cli_should_respect_top_limit() {
    let file = NamedTempFile::new().unwrap();
    write(
        file.path(),
        "apple one\napple two\napple three\napple four\n",
    )
    .unwrap();
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["apple", file.path().to_str().unwrap(), "--top", "25"])
        .assert()
        .success();
}

#[test]
fn cli_should_fail_when_file_does_not_exist() {
    Command::cargo_bin("fuzz")
        .unwrap()
        .args(["Lorem", "missing_file.txt"])
        .assert()
        .failure();
}
