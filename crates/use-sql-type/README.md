# use-sql-type

Common SQL type-name and type-modifier primitives for `RustUse`.

## Experimental

`use-sql-type` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_type::{SqlScalarType, SqlTypeName};

let scalar: SqlScalarType = "varchar".parse()?;
let name = SqlTypeName::new("NUMERIC")?;

assert_eq!(scalar, SqlScalarType::Text);
assert_eq!(name.as_str(), "NUMERIC");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Common SQL type labels.
- Lightweight type-name wrappers.
- Simple type modifiers.

## Non-goals

- Complete database-specific type systems.
- Type inference or casting behavior.

## License

Licensed under either Apache-2.0 or MIT.
