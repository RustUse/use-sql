#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_sql_ident::{SqlIdentifier, SqlIdentifierError};
use use_sql_table::SqlTableName;

/// SQL column name primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlColumnName(SqlIdentifier);

impl SqlColumnName {
    /// Creates a column name.
    ///
    /// # Errors
    ///
    /// Returns [`SqlColumnError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlColumnError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlColumnError::Identifier)
    }

    /// Returns the column name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SqlColumnName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlColumnName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SqlColumnName {
    type Err = SqlColumnError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// SQL column alias primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlColumnAlias(SqlIdentifier);

impl SqlColumnAlias {
    /// Creates a column alias.
    ///
    /// # Errors
    ///
    /// Returns [`SqlColumnError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlColumnError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlColumnError::Identifier)
    }

    /// Returns the alias text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SqlColumnAlias {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// SQL column reference metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlColumnRef {
    table: Option<SqlTableName>,
    name: SqlColumnName,
    alias: Option<SqlColumnAlias>,
}

impl SqlColumnRef {
    /// Creates an unqualified column reference.
    #[must_use]
    pub const fn new(name: SqlColumnName) -> Self {
        Self {
            table: None,
            name,
            alias: None,
        }
    }

    /// Creates a table-qualified column reference.
    #[must_use]
    pub const fn qualified(table: SqlTableName, name: SqlColumnName) -> Self {
        Self {
            table: Some(table),
            name,
            alias: None,
        }
    }

    /// Adds a column alias.
    #[must_use]
    pub fn with_alias(mut self, alias: SqlColumnAlias) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Returns the optional table qualifier.
    #[must_use]
    pub const fn table(&self) -> Option<&SqlTableName> {
        self.table.as_ref()
    }

    /// Returns the column name.
    #[must_use]
    pub const fn name(&self) -> &SqlColumnName {
        &self.name
    }

    /// Returns the optional alias.
    #[must_use]
    pub const fn alias(&self) -> Option<&SqlColumnAlias> {
        self.alias.as_ref()
    }
}

impl fmt::Display for SqlColumnRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(table) = &self.table {
            write!(formatter, "{table}.")?;
        }
        write!(formatter, "{}", self.name)?;
        if let Some(alias) = &self.alias {
            write!(formatter, " AS {alias}")?;
        }
        Ok(())
    }
}

/// Error returned when SQL column metadata is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SqlColumnError {
    Identifier(SqlIdentifierError),
}

impl fmt::Display for SqlColumnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(error) => write!(formatter, "invalid SQL column identifier: {error}"),
        }
    }
}

impl Error for SqlColumnError {}

#[cfg(test)]
mod tests {
    use super::{SqlColumnAlias, SqlColumnError, SqlColumnName, SqlColumnRef};
    use use_sql_table::SqlTableName;

    #[test]
    fn creates_column_references() -> Result<(), Box<dyn std::error::Error>> {
        let column =
            SqlColumnRef::qualified(SqlTableName::new("users")?, SqlColumnName::new("id")?)
                .with_alias(SqlColumnAlias::new("user_id")?);

        assert_eq!(column.to_string(), "users.id AS user_id");
        Ok(())
    }

    #[test]
    fn rejects_invalid_column_names() {
        assert!(matches!(
            SqlColumnName::new(""),
            Err(SqlColumnError::Identifier(_))
        ));
    }
}
