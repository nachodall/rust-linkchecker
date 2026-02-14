# Linkchecker

A concurrent Rust-based tool designed to validate URLs within Markdown files, extract page titles, and generate an updated report. This project is part of the LambdaClass Engineering Residency.

## Description

The **Linkchecker** parses an input Markdown file to extract all contained URLs. For each URL, the program performs an asynchronous HTTP request to retrieve the page's HTML body and extract the `<title>` tag. 

### Key Requirements
- **Strict Concurrency:** The program processes exactly 32 URLs at a time using asynchronous tasks.
- **Markdown Output:** Generates a new file where links are formatted as `[ EXTRACTED_TITLE ] ( URL )` for successes, or `[ HUMAN READABLE ERROR CODE ] ( URL )` for failures.
- **Crates:** Built using `reqwest` for HTTP networking and `tokio` for the async runtime.

## Getting Started

### Prerequisites
- **Rust** (installed via `asdf` or `rustup`).
- **Make** (for executing build tasks).

### Installation & Build
To set up the project and compile the binary:

```bash
make build
```

### Usage
Run the program by passing a Markdown file as an argument:

```bash
make run ARGS="path/to/your/file.md"
```

### Testing
To run the test suite:

```bash
make test
```

## Development Workflow

This repository adheres to the following workflow required by the residency:

1. **Branching Strategy:**
   - `main`: Contains stable, reviewed code.
   - `dev`: The primary branch for active development integration.
   - `feature/issue-name`: Temporary branches created for specific issues, branched from and merged into `dev`.
2. **Pull Requests:**
   - Every feature begins with a branch and a **Draft PR** toward `dev` to allow for early discussion and visibility.
   - Final code is merged from `dev` to `main` only after the MVP is fully achieved.
3. **Continuous Integration (CI):**
   - GitHub Actions are configured to enforce code formatting (`cargo fmt`), linting (`clippy`), and successful test completion on every push.

## Project Roadmap (Issues)
The implementation is broken down into the following stages:
- [ ] Setup repository skeleton and CI pipeline.
- [ ] Implement Markdown parsing and URL extraction.
- [ ] Develop the concurrent HTTP engine (32 simultaneous requests).
- [ ] Implement HTML title extraction and error handling.
- [ ] Finalize Markdown output formatting.

## License
This project is licensed under the MIT License.
