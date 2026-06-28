#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: ./setup.sh <crate-name>"
    echo "Example: ./setup.sh cronlint"
    exit 1
fi

NAME="$1"
SHIFTED_NAME="${NAME//-/_}"

# Rename crate in Cargo.toml
sed -i "s/rust-lib-template/${NAME}/g" Cargo.toml

# Rename binary target in src/bin/
mv "src/bin/rust-lib-template.rs" "src/bin/${NAME}.rs"
sed -i "s/rust_lib_template/${SHIFTED_NAME}/g" "src/bin/${NAME}.rs"

# Rename library references in src/
sed -i "s/rust_lib_template/${SHIFTED_NAME}/g" src/lib.rs

# Rename in Dockerfile
sed -i "s/rust-lib-template/${NAME}/g" Dockerfile

# Rename in Makefile
sed -i "s/rust-lib-template/${NAME}/g" Makefile

# Rename in CI
sed -i "s/rust-lib-template/${NAME}/g" .github/workflows/ci.yml

# Update README references
sed -i "s/rust-lib-template/${NAME}/g" README.md
sed -i "s/rust-lib/${NAME}/g" README.md

echo "Renamed crate to '${NAME}'"
echo "Next steps:"
echo "  1. Review the changes"
echo "  2. Delete src/example.rs and replace with your own modules"
echo "  3. Update src/lib.rs to export your modules"
echo "  4. cargo test && cargo clippy"

# Self-delete
rm -- "$0"