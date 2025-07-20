#!/bin/bash
set -euo pipefail

echo "Setting up pre-commit hooks..."

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Install the git hook scripts
pre-commit install

# Optionally run against all files
echo "Running pre-commit against all files..."
pre-commit run --all-files

echo "Pre-commit hooks installed successfully!"
