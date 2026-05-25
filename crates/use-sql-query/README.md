# use-sql-query

SQL query kind, intent, safety, and classification primitives for `RustUse`.

## Experimental

`use-sql-query` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_query::SqlQueryKind;

let kind: SqlQueryKind = "delete".parse()?;

assert!(SqlQueryKind::Select.is_read());
assert!(kind.is_write());
assert!(kind.is_destructive());
# Ok::<(), use_sql_query::SqlQueryKindParseError>(())
```

## Scope

- Query kind labels.
- Read, write, schema-change, and destructive classification helpers.
- Conservative first-token classification.

## Non-goals

- SQL parsing.
- Query execution.
- Authorization or policy enforcement.

## License

Licensed under either Apache-2.0 or MIT.
