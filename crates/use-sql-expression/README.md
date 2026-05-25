# use-sql-expression

Lightweight SQL expression and predicate primitives for `RustUse`.

## Experimental

`use-sql-expression` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_column::{SqlColumnName, SqlColumnRef};
use use_sql_expression::SqlExpression;

let expression = SqlExpression::column(SqlColumnRef::new(SqlColumnName::new("id")?));

assert_eq!(expression.to_string(), "id");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Identifier, column, value, and parameter expressions.
- Simple operator expression containers.
- Predicate metadata.

## Non-goals

- Full SQL ASTs.
- SQL parsing.
- Query-builder frameworks.

## License

Licensed under either Apache-2.0 or MIT.
