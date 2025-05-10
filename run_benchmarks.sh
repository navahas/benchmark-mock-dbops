#!/usr/bin/env bash

set -e

echo "Running benchmarks with high sample-size..."
cargo bench -- --sample-size 500

echo "Preparing report directory..."
rm -rf docs
mkdir -p docs

echo "Copying benchmark reports to docs/report..."
cp -r target/criterion/* docs/

echo "Done. You can now push to GitHub and view the report at:"
echo "https://<your-username>.github.io/<your-repo-name>/report/index.html"
