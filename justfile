alias b := build

# Build in release mode
build-release:
    cargo build --release

# Build in debug mode
build:
    cargo build

# Run Cargo Tests
test-cargo:
    cargo test

# Run Python Tests
test-py:
    ./test.py

test-turlu:
    #!/bin/bash
    cd turlu
    cargo r

# Run All Tests
test: test-cargo test-py test-turlu

