use use_sql::{
    clause, column, constraint, dialect, expression, ident, keyword, operator, param, query,
    schema, table, ty, value,
};

#[test]
fn facade_aliases_are_available() -> Result<(), Box<dyn std::error::Error>> {
    let identifier = ident::SqlIdentifier::new("users")?;
    assert_eq!(identifier.as_str(), "users");
    assert!(keyword::is_common_keyword("select"));
    assert_eq!(
        "postgres".parse::<dialect::SqlDialect>()?,
        dialect::SqlDialect::PostgreSql
    );
    assert_eq!(
        "varchar".parse::<ty::SqlScalarType>()?,
        ty::SqlScalarType::Text
    );
    assert_eq!(value::SqlValue::string("Ada").to_string(), "'Ada'");
    assert_eq!("$1".parse::<param::SqlParameter>()?.to_string(), "$1");

    let schema_name = schema::SqlSchemaName::new("public")?;
    let table_name = table::SqlTableName::new("users")?;
    let table_ref = table::SqlTableRef::new(table_name.clone()).with_schema(schema_name);
    let column_ref = column::SqlColumnRef::qualified(table_name, column::SqlColumnName::new("id")?);

    assert_eq!(table_ref.to_string(), "public.users");
    assert_eq!(column_ref.to_string(), "users.id");
    assert_eq!(
        "is not null".parse::<operator::SqlOperator>()?.to_string(),
        "IS NOT NULL"
    );
    assert_eq!(
        expression::SqlExpression::column(column_ref).to_string(),
        "users.id"
    );
    assert!(clause::SqlClauseKind::Where.comes_after(clause::SqlClauseKind::From));
    assert!(query::SqlQueryKind::Drop.is_destructive());
    assert_eq!(constraint::SqlConstraintKind::Unique.to_string(), "UNIQUE");

    Ok(())
}
