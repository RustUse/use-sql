#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_sql_ident::{SqlIdentifier, SqlIdentifierError};

/// SQL schema name primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlSchemaName(SqlIdentifier);

impl SqlSchemaName {
    /// Creates a schema name.
    ///
    /// # Errors
    ///
    /// Returns [`SqlSchemaError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlSchemaError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlSchemaError::Identifier)
    }

    /// Returns the schema name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SqlSchemaName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlSchemaName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SqlSchemaName {
    type Err = SqlSchemaError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// SQL database/catalog name primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlDatabaseName(SqlIdentifier);

impl SqlDatabaseName {
    /// Creates a database name.
    ///
    /// # Errors
    ///
    /// Returns [`SqlSchemaError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlSchemaError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlSchemaError::Identifier)
    }

    /// Returns the database name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SqlDatabaseName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Generic SQL namespace metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlNamespace {
    database: Option<SqlDatabaseName>,
    schema: SqlSchemaName,
}

impl SqlNamespace {
    /// Creates namespace metadata from a schema name.
    #[must_use]
    pub const fn new(schema: SqlSchemaName) -> Self {
        Self {
            database: None,
            schema,
        }
    }

    /// Adds a database/catalog qualifier.
    #[must_use]
    pub fn with_database(mut self, database: SqlDatabaseName) -> Self {
        self.database = Some(database);
        self
    }

    /// Returns the optional database/catalog name.
    #[must_use]
    pub const fn database(&self) -> Option<&SqlDatabaseName> {
        self.database.as_ref()
    }

    /// Returns the schema name.
    #[must_use]
    pub const fn schema(&self) -> &SqlSchemaName {
        &self.schema
    }
}

impl fmt::Display for SqlNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(database) = &self.database {
            write!(formatter, "{database}.")?;
        }
        write!(formatter, "{}", self.schema)
    }
}

/// Generic SQL search-path metadata.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlSearchPath {
    schemas: Vec<SqlSchemaName>,
}

impl SqlSearchPath {
    /// Creates a search path from schema names.
    #[must_use]
    pub const fn new(schemas: Vec<SqlSchemaName>) -> Self {
        Self { schemas }
    }

    /// Returns the schema list.
    #[must_use]
    pub fn schemas(&self) -> &[SqlSchemaName] {
        &self.schemas
    }

    /// Appends a schema to the search path.
    pub fn push(&mut self, schema: SqlSchemaName) {
        self.schemas.push(schema);
    }
}

impl fmt::Display for SqlSearchPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut schemas = self.schemas.iter();
        if let Some(first) = schemas.next() {
            write!(formatter, "{first}")?;
        }
        for schema in schemas {
            write!(formatter, ", {schema}")?;
        }
        Ok(())
    }
}

/// Error returned when SQL schema metadata is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SqlSchemaError {
    Identifier(SqlIdentifierError),
}

impl fmt::Display for SqlSchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(error) => write!(formatter, "invalid SQL schema identifier: {error}"),
        }
    }
}

impl Error for SqlSchemaError {}

#[cfg(test)]
mod tests {
    use super::{SqlDatabaseName, SqlNamespace, SqlSchemaError, SqlSchemaName, SqlSearchPath};

    #[test]
    fn creates_schema_names_and_paths() -> Result<(), SqlSchemaError> {
        let schema = SqlSchemaName::new("public")?;
        let namespace =
            SqlNamespace::new(schema.clone()).with_database(SqlDatabaseName::new("app")?);
        let path = SqlSearchPath::new(vec![schema]);

        assert_eq!(namespace.to_string(), "app.public");
        assert_eq!(path.to_string(), "public");
        Ok(())
    }
}
