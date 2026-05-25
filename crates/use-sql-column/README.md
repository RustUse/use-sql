# use-sql-column

SQL column name, reference, table qualification, and alias primitives for `RustUse`.

## Experimental

`use-sql-column` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_column::{SqlColumnName, SqlColumnRef};
use use_sql_table::SqlTableName;

let column = SqlColumnRef::qualified(SqlTableName::new("users")?, SqlColumnName::new("id")?);

assert_eq!(column.to_string(), "users.id");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Column names.
- Optional table-qualified column references.
- Column aliases.

## Non-goals

- Projection building.
- Database introspection.

## License

Licensed under either Apache-2.0 or MIT.
