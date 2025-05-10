#!/usr/bin/env bash

set -e

echo "Running benchmarks..."
cargo bench

echo "Preparing report directory..."
rm -rf docs/report
mkdir -p docs/report

echo "Copying benchmark reports to docs/report..."
cp -r cp -r target/criterion/report/* docs/report/

echo "Done. You can push to GitHub and view the report at:"
echo "https://<your-username>.github.io/<your-repo-name>/report/index.html"
