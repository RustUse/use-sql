# use-sql-value

Simple SQL literal and value primitives for `RustUse`.

## Experimental

`use-sql-value` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_value::{SqlNumberLiteral, SqlValue};

let string_value = SqlValue::string("Ada's account");
let number = SqlNumberLiteral::new("42")?;

assert_eq!(string_value.to_string(), "'Ada''s account'");
assert_eq!(number.to_string(), "42");
# Ok::<(), use_sql_value::SqlValueError>(())
```

## Scope

- Null, string, number, and boolean literal primitives.
- Simple literal rendering for controlled values.

## Non-goals

- Query construction from user input.
- Database driver binding behavior.
- SQL parser or formatter behavior.

Prefer parameters for user-provided values.

## License

Licensed under either Apache-2.0 or MIT.
