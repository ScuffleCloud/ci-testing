export RUST_TOOLCHAIN := "nightly"

test *args:
    #!/bin/bash
    set -e -o pipefail 

    INSTA_FORCE_PASS=1 cargo llvm-cov clean --workspace
    INSTA_FORCE_PASS=1 cargo llvm-cov nextest --branch --include-build-script --no-report {{args}}

    # Do not generate the coverage report on CI
    cargo insta review
    cargo llvm-cov report --html
    cargo llvm-cov report --lcov --output-path ./lcov.info

test-ci *args:
    CI=1 cargo llvm-cov nextest --lcov --profile ci --output-path ./lcov.info {{args}}

coverage:
    http-server ./target/llvm-cov/html/
