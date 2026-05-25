# RustUse/use-sql

`use-sql` is a RustUse facade workspace for small, focused SQL primitive crates. It models generic SQL language, data-model, query-classification, identifier, value, parameter, operator, clause, and constraint vocabulary without becoming a database access layer.

## Experimental

`use-sql` is experimental while the workspace remains below `0.3.0`. Expect small API refinements during the first release wave.

## Non-goals

`use-sql` is not:

- a database driver
- an ORM
- a query builder framework
- a migration engine
- a SQL parser
- a SQL formatter
- a database client

The crates avoid network calls, database connections, query execution, ORM behavior, migration behavior, full SQL parsing, and full SQL formatting. Rendering is limited to small `Display` implementations for primitive values and labels.

## Relationships

- `use-sql` models generic SQL language, data-model, and query primitives.
- `use-database` may model generic database system concepts separately.
- `use-postgres`, `use-sqlite`, `use-mysql`, and similar facades should model database-specific behavior separately.
- Database-specific facades can reuse `use-sql` primitives later without moving driver or client behavior into this workspace.

## Workspace crates

| Crate                | Path                         | Purpose                                                                      |
| -------------------- | ---------------------------- | ---------------------------------------------------------------------------- |
| `use-sql`            | `crates/use-sql/`            | Feature-gated facade over the focused SQL primitive crates                   |
| `use-sql-ident`      | `crates/use-sql-ident/`      | SQL identifiers, aliases, qualified names, quoting checks, and normalization |
| `use-sql-keyword`    | `crates/use-sql-keyword/`    | Common SQL keyword vocabulary and reserved-like checks                       |
| `use-sql-dialect`    | `crates/use-sql-dialect/`    | Lightweight SQL dialect and dialect-family labels                            |
| `use-sql-type`       | `crates/use-sql-type/`       | Common SQL type-name and modifier primitives                                 |
| `use-sql-value`      | `crates/use-sql-value/`      | Simple SQL literal and value primitives                                      |
| `use-sql-param`      | `crates/use-sql-param/`      | SQL parameter placeholder styles, names, indexes, parsing, and rendering     |
| `use-sql-table`      | `crates/use-sql-table/`      | Table names, table references, optional schemas, and aliases                 |
| `use-sql-column`     | `crates/use-sql-column/`     | Column names, column references, optional table qualification, and aliases   |
| `use-sql-schema`     | `crates/use-sql-schema/`     | Schema, database, namespace, and search-path primitives                      |
| `use-sql-operator`   | `crates/use-sql-operator/`   | Comparison, logical, null, and pattern operator labels                       |
| `use-sql-expression` | `crates/use-sql-expression/` | Lightweight expression and predicate containers                              |
| `use-sql-clause`     | `crates/use-sql-clause/`     | Clause labels and common clause-order helpers                                |
| `use-sql-query`      | `crates/use-sql-query/`      | Query kind, intent, safety, and conservative classification helpers          |
| `use-sql-constraint` | `crates/use-sql-constraint/` | Schema constraint names, kinds, and metadata                                 |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

```toml
[dependencies]
use-sql = { git = "https://github.com/RustUse/use-sql", rev = "<commit>" }
```

After publication, choose the narrowest focused crate that matches your use case or use the facade when one dependency is more convenient.

```toml
[dependencies]
use-sql = "0.1.0"
```

## Basic usage

```rust
use use_sql::column::{SqlColumnName, SqlColumnRef};
use use_sql::constraint::SqlConstraintKind;
use use_sql::dialect::SqlDialect;
use use_sql::ident::{SqlIdentifier, needs_quoting};
use use_sql::param::SqlParameter;
use use_sql::query::SqlQueryKind;
use use_sql::table::{SqlTableName, SqlTableRef};

let ident = SqlIdentifier::new("users")?;
assert_eq!(ident.as_str(), "users");
assert!(needs_quoting("select"));

let table = SqlTableRef::new(SqlTableName::new("users")?);
let column = SqlColumnRef::qualified(SqlTableName::new("users")?, SqlColumnName::new("id")?);

let postgres_param: SqlParameter = "$1".parse()?;
let positional_param: SqlParameter = "?".parse()?;
let kind = SqlQueryKind::Select;
let constraint = SqlConstraintKind::PrimaryKey;
let dialect: SqlDialect = "postgres".parse()?;

assert_eq!(table.to_string(), "users");
assert_eq!(column.to_string(), "users.id");
assert_eq!(postgres_param.to_string(), "$1");
assert_eq!(positional_param.to_string(), "?");
assert!(kind.is_read());
assert!(!kind.is_write());
assert!(!kind.is_destructive());
assert_eq!(constraint.to_string(), "PRIMARY KEY");
assert_eq!(dialect, SqlDialect::PostgreSql);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

`use-sql` keeps focused crates small, dependency-light, deterministic, and framework-free. The workspace favors validated newtypes, explicit error enums, stable display labels, conservative parsing helpers, and small metadata containers.

The SQL expression crate intentionally stays shallow. It can hold identifiers, values, parameters, and simple operator relationships, but it does not parse SQL text or construct complete queries.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo check --workspace --all-features --examples
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
