# Rust Coverage Testing Example

This is an example of how to use the `cargo-llvm-cov` tool to generate coverage reports for a Rust project.

## Setup

```bash
cargo install binstall
```

We need to use nightly Rust to run the tests and generate the coverage report.

```bash
rustup install nightly
```

Once you have `binstall` installed, you can install the other tools with the following command:

```bash
cargo binstall -y cargo-nextest cargo-llvm-cov cargo-mutants just
```

## Running the tests

```bash
just test
```

## See the coverage report in your browser

```bash
just coverage
```
