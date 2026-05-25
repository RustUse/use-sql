# RustUse SQL Crate Template

Use this checklist when adding a focused crate to the `use-sql` workspace.

## Checklist

- Keep package metadata inherited from the workspace wherever possible.
- Prefer no dependencies for primitive SQL vocabulary crates.
- Use README-driven crate docs with `#![doc = include_str!("../README.md")]`.
- Use explicit error enums instead of stringly errors.
- Add unit tests for accepted and rejected values.
- Keep drivers, clients, ORM behavior, migrations, full parsers, full formatters, and query execution out of scope.

## Validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```
