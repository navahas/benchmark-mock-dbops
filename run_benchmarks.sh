#!/usr/bin/env bash

set -e

echo "Running benchmarks with high sample-size..."
cargo bench -- --sample-size 500

echo "Preparing report directory..."
rm -rf docs
mkdir -p docs

echo "Copying benchmark reports to docs/report..."
cp -r target/criterion/* docs/

echo "To see in browser open docs/report/index.html"
