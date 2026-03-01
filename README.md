# Linkchecker

A concurrent Rust-based tool designed to validate URLs within Markdown files, extract page titles, and generate an updated report. This project is part of the LambdaClass Engineering Residency.

## Description

The **Linkchecker** parses an input Markdown file to extract all contained URLs. For each URL, the program performs an asynchronous HTTP request using a concurrency limit of 32 simultaneous workers to retrieve the page's HTML body and extract the `<title>` tag. 

The execution provides a terminal summary and writes the full report to a new file named `output.md` in the current directory.

### Report Format
The generated `output.md` will list each URL in the following format:
* **Success:** `[ Page Title ] ( https://example.com )`
* **Failure:** `[ Error Message or HTTP Code ] ( https://example.com/404 )`

## 1. Getting Started

### Prerequisites
- **Rust** 1.80.0 +.

### 2. Installation & Build
To set up the project and compile the release binary:

```bash
make build
```

### 3. Usage

Run the program by passing a Markdown file as an argument:
```Bash
make run ARGS="path/to/your/file.md"
```

### 4. Testing

To run the test suite, which includes HTML parsing and async network mocked validations:
```Bash
make test
```
License

This project is licensed under the MIT License.
