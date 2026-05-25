# use-sql-clause

SQL clause label and ordering primitives for `RustUse`.

## Experimental

`use-sql-clause` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_clause::SqlClauseKind;

assert!(SqlClauseKind::Where.comes_after(SqlClauseKind::From));
assert_eq!(SqlClauseKind::OrderBy.to_string(), "ORDER BY");
```

## Scope

- Common clause labels.
- Conservative clause ordering helpers.

## Non-goals

- SQL query parsing.
- Query formatting.
- Query-builder behavior.

## License

Licensed under either Apache-2.0 or MIT.
