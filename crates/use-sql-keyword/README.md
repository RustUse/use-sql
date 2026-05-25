# use-sql-keyword

Common SQL keyword vocabulary primitives for `RustUse`.

## Experimental

`use-sql-keyword` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_keyword::{SqlKeyword, SqlKeywordKind, is_common_keyword};

let keyword: SqlKeyword = "select".parse()?;

assert_eq!(keyword.kind(), SqlKeywordKind::DataQuery);
assert!(is_common_keyword("where"));
# Ok::<(), use_sql_keyword::SqlKeywordParseError>(())
```

## Scope

- Common SQL keywords.
- Reserved-like checks for conservative identifier helpers.
- Broad keyword categories.

## Non-goals

- Exhaustive SQL standard keyword coverage.
- Dialect-specific reserved-word tables.

## License

Licensed under either Apache-2.0 or MIT.
