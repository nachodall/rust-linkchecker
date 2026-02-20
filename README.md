# Linkchecker

A concurrent Rust-based tool designed to validate URLs within Markdown files, extract page titles, and generate an updated report. This project is part of the LambdaClass Engineering Residency.

## Description

The **Linkchecker** parses an input Markdown file to extract all contained URLs. For each URL, the program performs an asynchronous HTTP request to retrieve the page's HTML body and extract the `<title>` tag. 


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


## License
This project is licensed under the MIT License.
