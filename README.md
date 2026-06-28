# Rust Library Template

A GitHub template for publishable Rust library crates — structured types, error handling, serde, tests, CI, and Docker included. The library is the primary deliverable; the binary is a thin optional wrapper.

## Why not rust-cli?

`rust-cli` is for command-line tools where the binary is the product. Libraries are different:
- The crate itself is published to crates.io for others to depend on
- The public API surface (`pub fn`, `pub struct`) is the deliverable
- A thin binary is optional, mainly for ad-hoc usage or demo
- Error types use `thiserror` for clean, idiomatic Rust error chains
- Serde derives are baked in — libraries that handle data need serialization

## Features

- **Library-first** — `lib.rs` is the main module, binary is a thin wrapper
- **Error handling** with [thiserror](https://docs.rs/thiserror) — clean, idiomatic error types
- **Serialization** with [serde](https://docs.rs/serde) + [serde_json](https://docs.rs/serde_json)
- **Tests** — unit tests in `src/`, integration tests in `tests/`
- **CI** via GitHub Actions (test, clippy, fmt, build on push/PR)
- **Docker** multi-stage build (scratch image, static binary)
- **Makefile** for common tasks

## Usage

1. Click **"Use this template"** on GitHub to create a new repo
2. Run the setup script:
   ```sh
   ./setup.sh mylib
   ```
3. Replace `src/example.rs` with your own modules
4. Update `src/lib.rs` to re-export your modules
5. `cargo publish` when ready

## Project Structure

```
.
├── src/
│   ├── lib.rs                  # Public API, re-exports modules
│   ├── error.rs                # Error types (thiserror)
│   ├── example.rs              # Example module (delete me)
│   └── bin/
│       └── rust-lib-template.rs # Thin binary wrapper
├── tests/
│   └── integration_test.rs     # Integration tests
├── .github/
│   └── workflows/
│       └── ci.yml              # Test + clippy + fmt + Docker push
├── Cargo.toml
├── Dockerfile
├── Makefile
├── setup.sh
└── README.md
```

## Quick Start

```sh
# Run tests
make test

# Build release binary
make build

# Run the binary
make run

# Lint
make lint

# Build Docker image
make docker
```

## Container Images

CI builds and pushes a container image to GHCR on every push to any branch.

```sh
# Pull the latest image
docker pull ghcr.io/<owner>/rust-lib-template:latest

# Pull a specific commit
docker pull ghcr.io/<owner>/rust-lib-template:<sha>

# Run
docker run ghcr.io/<owner>/rust-lib-template:latest "hello"
```

Replace `<owner>` with your GitHub username or org. Images are tagged with both `latest` and the commit SHA.

## Publishing

```sh
# Dry run
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

Make sure `Cargo.toml` has the correct `name`, `version`, `description`, and `license` fields before publishing.

## Adding a New Module

```rust
// src/mymodule.rs
use crate::Error;

pub fn do_thing(input: &str) -> Result<String, Error> {
    Ok(format!("processed: {input}"))
}
```

Then add `pub mod mymodule;` to `src/lib.rs` and re-export as needed.

## License

MIT