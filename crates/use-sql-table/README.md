# use-sql-table

SQL table name, reference, schema, and alias primitives for `RustUse`.

## Experimental

`use-sql-table` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_schema::SqlSchemaName;
use use_sql_table::{SqlTableName, SqlTableRef};

let table = SqlTableRef::new(SqlTableName::new("users")?)
    .with_schema(SqlSchemaName::new("public")?);

assert_eq!(table.to_string(), "public.users");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Table names.
- Optional schema-qualified table references.
- Table aliases.

## Non-goals

- Query construction.
- Database introspection.

## License

Licensed under either Apache-2.0 or MIT.
