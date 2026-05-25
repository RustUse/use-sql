# use-sql

Feature-gated facade over the focused RustUse SQL primitive crates.

## Experimental

`use-sql` is experimental while the workspace remains below `0.3.0`.

## Example

```rust
# #[cfg(feature = "full")]
# {
use use_sql::ident::SqlIdentifier;
use use_sql::param::SqlParameter;
use use_sql::query::SqlQueryKind;

let ident = SqlIdentifier::new("users")?;
let param: SqlParameter = "$1".parse()?;
let kind = SqlQueryKind::Select;

assert_eq!(ident.as_str(), "users");
assert_eq!(param.to_string(), "$1");
assert!(kind.is_read());
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Facade access to SQL primitive crates.
- Optional features for narrow dependency surfaces.
- Lightweight aliases that keep child crate boundaries visible.

## Non-goals

- Database drivers or clients.
- ORM, migration, or query-builder behavior.
- Full SQL parsing or formatting.
- Query execution.

## License

Licensed under either Apache-2.0 or MIT.
