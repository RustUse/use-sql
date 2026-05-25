use use_sql::column::{SqlColumnName, SqlColumnRef};
use use_sql::constraint::SqlConstraintKind;
use use_sql::dialect::SqlDialect;
use use_sql::ident::{SqlIdentifier, needs_quoting};
use use_sql::param::SqlParameter;
use use_sql::query::SqlQueryKind;
use use_sql::table::{SqlTableName, SqlTableRef};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ident = SqlIdentifier::new("users")?;
    let table = SqlTableRef::new(SqlTableName::new("users")?);
    let column = SqlColumnRef::qualified(SqlTableName::new("users")?, SqlColumnName::new("id")?);
    let postgres_param: SqlParameter = "$1".parse()?;
    let positional_param: SqlParameter = "?".parse()?;
    let kind = SqlQueryKind::Select;
    let constraint = SqlConstraintKind::PrimaryKey;
    let dialect: SqlDialect = "postgres".parse()?;

    assert_eq!(ident.as_str(), "users");
    assert!(needs_quoting("select"));
    assert_eq!(table.to_string(), "users");
    assert_eq!(column.to_string(), "users.id");
    assert_eq!(postgres_param.to_string(), "$1");
    assert_eq!(positional_param.to_string(), "?");
    assert!(kind.is_read());
    assert_eq!(constraint.to_string(), "PRIMARY KEY");
    assert_eq!(dialect, SqlDialect::PostgreSql);

    Ok(())
}
