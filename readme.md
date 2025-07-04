# cargo check-tag 🔎 🏷️

A trivial cargo extension for CI workflows to check that the cargo manifest version matches the git tag you've just pushed

## Supports
- Version tags with or without `v` prefix
- Github Actions (via the `GITHUB_REF` env var)

## Usage
In your CI workflow:
- `cargo install cargo-check-tag`
- `cargo check-tag`
- `cargo publish`
