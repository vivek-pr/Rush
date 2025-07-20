#!/bin/bash
set -euo pipefail

echo "Generating test coverage report..."

# Install cargo-llvm-cov if not present
if ! cargo llvm-cov --version &> /dev/null; then
    echo "Installing cargo-llvm-cov..."
    cargo install cargo-llvm-cov
fi

# Clean previous coverage data
cargo llvm-cov clean --workspace

# Generate coverage report
cargo llvm-cov \
    --all-features \
    --workspace \
    --html \
    --lcov \
    --output-path target/llvm-cov/lcov.info

echo "Coverage report generated!"
echo "HTML report: target/llvm-cov/html/index.html"
echo "LCOV report: target/llvm-cov/lcov.info"
