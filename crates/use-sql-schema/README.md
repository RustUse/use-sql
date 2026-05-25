# use-sql-schema

SQL schema, database, namespace, and search-path primitives for `RustUse`.

## Experimental

`use-sql-schema` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_schema::{SqlSchemaName, SqlSearchPath};

let public = SqlSchemaName::new("public")?;
let search_path = SqlSearchPath::new(vec![public.clone()]);

assert_eq!(public.as_str(), "public");
assert_eq!(search_path.to_string(), "public");
# Ok::<(), use_sql_schema::SqlSchemaError>(())
```

## Scope

- Generic schema and database names.
- Namespace metadata.
- Search-path lists.

## Non-goals

- PostgreSQL-specific search-path behavior.
- Migration or introspection behavior.

## License

Licensed under either Apache-2.0 or MIT.
