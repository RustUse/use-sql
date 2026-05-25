# use-sql-ident

SQL identifier, alias, qualified-name, quoting, and normalization primitives for `RustUse`.

## Experimental

`use-sql-ident` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_ident::{SqlIdentifier, needs_quoting, quote_ident};

let ident = SqlIdentifier::new("users")?;

assert_eq!(ident.as_str(), "users");
assert!(needs_quoting("select"));
assert_eq!(quote_ident("order items"), "\"order items\"");
# Ok::<(), use_sql_ident::SqlIdentifierError>(())
```

## Scope

- Conservative SQL identifier text validation.
- Unquoted identifier checks.
- Quoting and simple normalization helpers.
- Qualified names and aliases.

## Non-goals

- Complete dialect-specific identifier rules.
- SQL parsing or formatting.

## License

Licensed under either Apache-2.0 or MIT.
