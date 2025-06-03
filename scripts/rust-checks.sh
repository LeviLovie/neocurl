#!/bin/bash
set -euo pipefail

echo "Running cargo fmt --check..."
cargo fmt --all --check

echo "Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running cargo check..."
cargo check --all-targets --all-features

echo "All checks passed!"

