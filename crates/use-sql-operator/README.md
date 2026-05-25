# use-sql-operator

Common SQL comparison, logical, null, and pattern operator primitives for `RustUse`.

## Experimental

`use-sql-operator` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_operator::{SqlComparisonOperator, SqlOperator};

let operator: SqlOperator = "is not null".parse()?;

assert_eq!(SqlComparisonOperator::LessOrEqual.to_string(), "<=");
assert_eq!(operator.to_string(), "IS NOT NULL");
# Ok::<(), use_sql_operator::SqlOperatorParseError>(())
```

## Scope

- Comparison operators.
- Logical operators.
- Null and pattern operators.

## Non-goals

- Operator precedence handling.
- Full expression parsing.

## License

Licensed under either Apache-2.0 or MIT.
