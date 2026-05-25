# use-sql-constraint

SQL constraint kind, name, and schema metadata primitives for `RustUse`.

## Experimental

`use-sql-constraint` is experimental while `use-sql` remains below `0.3.0`.

## Example

```rust
use use_sql_constraint::{SqlConstraint, SqlConstraintKind};

let constraint = SqlConstraint::new(SqlConstraintKind::PrimaryKey);

assert_eq!(constraint.kind(), SqlConstraintKind::PrimaryKey);
assert_eq!(constraint.to_string(), "PRIMARY KEY");
```

## Scope

- Primary key, foreign key, unique, not null, check, default, and generated labels.
- Optional constraint names.
- Schema metadata containers.

## Non-goals

- Migration engines.
- DDL generation frameworks.
- Database introspection.

## License

Licensed under either Apache-2.0 or MIT.
