# Contributing to RustUse/use-sql

Thank you for helping improve `use-sql`.

This workspace keeps SQL utilities small, primitive, dependency-light, and framework-free. Contributions should avoid database connections, network calls, query execution, ORM behavior, migration behavior, full SQL parsing, and full SQL formatting.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Prefer focused crates under `crates/` for implementation and keep the `use-sql` facade thin.
