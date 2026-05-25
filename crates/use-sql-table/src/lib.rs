#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_sql_ident::{SqlIdentifier, SqlIdentifierError};
use use_sql_schema::SqlSchemaName;

/// SQL table name primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlTableName(SqlIdentifier);

impl SqlTableName {
    /// Creates a table name.
    ///
    /// # Errors
    ///
    /// Returns [`SqlTableError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlTableError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlTableError::Identifier)
    }

    /// Returns the table name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SqlTableName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlTableName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SqlTableName {
    type Err = SqlTableError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// SQL table alias primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlTableAlias(SqlIdentifier);

impl SqlTableAlias {
    /// Creates a table alias.
    ///
    /// # Errors
    ///
    /// Returns [`SqlTableError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlTableError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlTableError::Identifier)
    }

    /// Returns the alias text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SqlTableAlias {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// SQL table reference metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlTableRef {
    schema: Option<SqlSchemaName>,
    name: SqlTableName,
    alias: Option<SqlTableAlias>,
}

impl SqlTableRef {
    /// Creates a table reference from a table name.
    #[must_use]
    pub const fn new(name: SqlTableName) -> Self {
        Self {
            schema: None,
            name,
            alias: None,
        }
    }

    /// Adds schema qualification.
    #[must_use]
    pub fn with_schema(mut self, schema: SqlSchemaName) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Adds a table alias.
    #[must_use]
    pub fn with_alias(mut self, alias: SqlTableAlias) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Returns the optional schema name.
    #[must_use]
    pub const fn schema(&self) -> Option<&SqlSchemaName> {
        self.schema.as_ref()
    }

    /// Returns the table name.
    #[must_use]
    pub const fn name(&self) -> &SqlTableName {
        &self.name
    }

    /// Returns the optional alias.
    #[must_use]
    pub const fn alias(&self) -> Option<&SqlTableAlias> {
        self.alias.as_ref()
    }
}

impl fmt::Display for SqlTableRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(schema) = &self.schema {
            write!(formatter, "{schema}.")?;
        }
        write!(formatter, "{}", self.name)?;
        if let Some(alias) = &self.alias {
            write!(formatter, " AS {alias}")?;
        }
        Ok(())
    }
}

/// Error returned when SQL table metadata is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SqlTableError {
    Identifier(SqlIdentifierError),
}

impl fmt::Display for SqlTableError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(error) => write!(formatter, "invalid SQL table identifier: {error}"),
        }
    }
}

impl Error for SqlTableError {}

#[cfg(test)]
mod tests {
    use super::{SqlTableAlias, SqlTableError, SqlTableName, SqlTableRef};
    use use_sql_schema::SqlSchemaName;

    #[test]
    fn creates_table_references() -> Result<(), Box<dyn std::error::Error>> {
        let table = SqlTableRef::new(SqlTableName::new("users")?)
            .with_schema(SqlSchemaName::new("public")?)
            .with_alias(SqlTableAlias::new("u")?);

        assert_eq!(table.to_string(), "public.users AS u");
        Ok(())
    }

    #[test]
    fn rejects_invalid_table_names() {
        assert!(matches!(
            SqlTableName::new(""),
            Err(SqlTableError::Identifier(_))
        ));
    }
}
