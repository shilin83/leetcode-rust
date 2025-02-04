#!/usr/bin/env bash

set -e

if ! which -v cargo-llvm-cov >/dev/null; then
    cargo install cargo-llvm-cov
    rustup component add llvm-tools-preview
fi

cargo llvm-cov --open
