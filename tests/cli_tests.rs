use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn link_checker_fails_when_file_does_not_exist() {
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg("ghost-file.md");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn link_checker_works_with_valid_input_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "C [Google](https://www.google.com)").unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("https://www.google.com"))
        .stdout(predicate::str::contains("OK"));
}
