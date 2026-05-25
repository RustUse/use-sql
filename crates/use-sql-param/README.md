# use-sql-param

SQL parameter placeholder style, index, and name primitives for `RustUse`.

## Experimental

`use-sql-param` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_param::{SqlParameter, SqlParameterStyle};

let indexed: SqlParameter = "$1".parse()?;
let positional: SqlParameter = "?".parse()?;

assert_eq!(indexed.style(), SqlParameterStyle::PostgresIndexed);
assert_eq!(positional.to_string(), "?");
# Ok::<(), use_sql_param::SqlParameterError>(())
```

## Scope

- PostgreSQL-style indexed parameters.
- Positional question-mark parameters.
- Named colon and at-sign parameters.

## Non-goals

- Binding values to database drivers.
- Query execution.
- SQL parsing beyond placeholder strings.

## License

Licensed under either Apache-2.0 or MIT.
