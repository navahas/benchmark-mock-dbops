#!/usr/bin/env bash

set -e

echo "Running benchmarks..."
cargo bench -- --sample-size 500

echo "Preparing report directory..."
rm -rf docs/benchmarks
mkdir -p docs/benchmarks

echo "Copying benchmark reports to docs/report..."
cp -r target/criterion/* docs/benchmarks/

echo "Done. You can now push to GitHub and view the report at:"
echo "https://<your-username>.github.io/<your-repo-name>/report/index.html"
