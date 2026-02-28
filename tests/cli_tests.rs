use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::io::Write;
use tempfile::{NamedTempFile, tempdir};

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
    writeln!(file, "[Google](https://www.google.com)").unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success();

    let output_path = dir.path().join("output.md");
    assert!(output_path.exists());
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("https://www.google.com"));
    assert!(
        content.contains("[ Google ] ( https://www.google.com )")
            || content.contains("[ Google ] ( https://www.google.com/ )")
    );
}

#[test]
fn ignores_plain_text_files_without_markdown_urls() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "This is plain text, no links in here.").unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 0 links worked out of 0 total links checked.",
    ));

    let output_path = dir.path().join("output.md");
    assert!(output_path.exists());
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.is_empty());
}

#[test]
fn link_checker_prints_summary() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "[Google](https://www.google.com)").unwrap();
    writeln!(file, "[Invalid](https://invalid.domain.xyz)").unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

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

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 15 links worked out of 15 total links checked.",
    ));
}

#[test]
fn test_overwrites_existing_output_file_idempotency() {
    let mut input_file = NamedTempFile::new().unwrap();
    writeln!(input_file, "[Example](https://example.com)").unwrap();

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.md");

    std::fs::write(&output_path, "junk data that should be deleted").unwrap();

    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(input_file.path()).current_dir(dir.path());
    cmd.assert().success();

    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(
        !content.contains("junk data"),
        "The output file was not properly overwritten"
    );
    assert!(
        content.contains("https://example.com"),
        "The new content was not written"
    );
}

#[test]
fn test_complex_markdown_resilience_ignores_code_blocks() {
    let mut file = NamedTempFile::new().unwrap();
    let complex_md = r#"
# Intro
Here is a normal link: [Valid](https://www.rust-lang.org)

```markdown
This is inside a code block, should be ignored: [Fake](https://this-is-a-fake-link-in-code.com)

And an inline code link [Inline](https://inline-code-link.com)
"#;
    writeln!(file, "{}", complex_md).unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success();

    let output_path = dir.path().join("output.md");
    let content = std::fs::read_to_string(output_path).unwrap();

    assert!(content.contains("https://www.rust-lang.org"));
    assert!(
        !content.contains("https://this-is-a-fake-link-in-code.com"),
        "Parser extracted a link from a code block"
    );
    assert!(
        !content.contains("https://inline-code-link.com"),
        "Parser extracted a link from inline code"
    );
}

#[test]
fn test_graceful_handling_of_empty_input_file() {
    let file = NamedTempFile::new().unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "> [Summary] 0 links worked out of 0 total links checked.",
    ));

    let output_path = dir.path().join("output.md");
    let content = std::fs::read_to_string(output_path).unwrap();
    assert_eq!(content.trim(), "");
}

#[test]
fn test_handles_malformed_urls_without_panic() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "[Bad Protocol](httpx://bad-url)").unwrap();

    let dir = tempdir().unwrap();
    let mut cmd = cargo_bin_cmd!("rust-linkchecker");
    cmd.arg(file.path()).current_dir(dir.path());

    cmd.assert().success();

    let output_path = dir.path().join("output.md");
    let content = std::fs::read_to_string(output_path).unwrap();

    assert!(content.contains("[ Invalid URL ] ( httpx://bad-url )") || content.contains("error"));
}
