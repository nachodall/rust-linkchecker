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

#[test]
fn ignores_plain_text_files_without_markdown_urls() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "This is plain text, no links in here.").unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 0 links worked out of 0 total links checked.",
    ));
}

#[test]
fn link_checker_prints_summary() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "[Google](https://www.google.com)").unwrap();
    writeln!(file, "[Invalid](https://invalid.domain.xyz)").unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 1 links worked out of 2 total links checked.",
    ));
}

#[test]
fn processes_multiple_links_concurrently_and_sums_correctly() {
    let mut file = NamedTempFile::new().unwrap();
    for _ in 0..15 {
        writeln!(file, "[Rust](https://www.rust-lang.org/)").unwrap();
    }
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 15 links worked out of 15 total links checked.",
    ));
}
