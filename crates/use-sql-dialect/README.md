# use-sql-dialect

Lightweight SQL dialect and dialect-family labels for `RustUse`.

## Experimental

`use-sql-dialect` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_dialect::{SqlDialect, SqlDialectFamily};

let dialect: SqlDialect = "postgres".parse()?;

assert_eq!(dialect, SqlDialect::PostgreSql);
assert_eq!(dialect.family(), SqlDialectFamily::PostgreSql);
# Ok::<(), use_sql_dialect::SqlDialectParseError>(())
```

## Scope

- Dialect labels for common SQL systems.
- Lightweight dialect families.
- Display and case-insensitive parsing helpers.

## Non-goals

- Deep dialect behavior.
- Feature matrices or compatibility checks.

## License

Licensed under either Apache-2.0 or MIT.
