#!/bin/bash

set -e

if ! command -v cargo-llvm-cov; then
    cargo install cargo-llvm-cov

    rustup component add llvm-tools-preview
fi

cargo llvm-cov --lcov --output-path target/lcov.info

if ! command -v genhtml; then
    brew install lcov
fi

genhtml -o target/coverage target/lcov.info

if [ -d target/coverage ]; then
    open target/coverage/index.html
fi
