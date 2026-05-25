#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_sql_ident::{SqlIdentifier, SqlIdentifierError};

/// SQL constraint kind labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlConstraintKind {
    #[default]
    PrimaryKey,
    ForeignKey,
    Unique,
    NotNull,
    Check,
    Default,
    Generated,
}

impl SqlConstraintKind {
    /// Returns the stable constraint label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryKey => "PRIMARY KEY",
            Self::ForeignKey => "FOREIGN KEY",
            Self::Unique => "UNIQUE",
            Self::NotNull => "NOT NULL",
            Self::Check => "CHECK",
            Self::Default => "DEFAULT",
            Self::Generated => "GENERATED",
        }
    }
}

impl fmt::Display for SqlConstraintKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlConstraintKind {
    type Err = SqlConstraintError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_constraint(input)?.as_str() {
            "PRIMARY KEY" | "PRIMARY" => Ok(Self::PrimaryKey),
            "FOREIGN KEY" | "FOREIGN" => Ok(Self::ForeignKey),
            "UNIQUE" => Ok(Self::Unique),
            "NOT NULL" => Ok(Self::NotNull),
            "CHECK" => Ok(Self::Check),
            "DEFAULT" => Ok(Self::Default),
            "GENERATED" => Ok(Self::Generated),
            _ => Err(SqlConstraintError::UnknownKind),
        }
    }
}

/// SQL constraint name primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlConstraintName(SqlIdentifier);

impl SqlConstraintName {
    /// Creates a constraint name.
    ///
    /// # Errors
    ///
    /// Returns [`SqlConstraintError`] when identifier validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlConstraintError> {
        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlConstraintError::Identifier)
    }

    /// Returns the constraint name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SqlConstraintName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// SQL constraint metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlConstraint {
    kind: SqlConstraintKind,
    name: Option<SqlConstraintName>,
}

impl SqlConstraint {
    /// Creates constraint metadata from a kind.
    #[must_use]
    pub const fn new(kind: SqlConstraintKind) -> Self {
        Self { kind, name: None }
    }

    /// Adds a constraint name.
    #[must_use]
    pub fn with_name(mut self, name: SqlConstraintName) -> Self {
        self.name = Some(name);
        self
    }

    /// Returns the constraint kind.
    #[must_use]
    pub const fn kind(&self) -> SqlConstraintKind {
        self.kind
    }

    /// Returns the optional constraint name.
    #[must_use]
    pub const fn name(&self) -> Option<&SqlConstraintName> {
        self.name.as_ref()
    }
}

impl fmt::Display for SqlConstraint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(formatter, "CONSTRAINT {name} ")?;
        }
        write!(formatter, "{}", self.kind)
    }
}

/// Error returned when SQL constraint metadata is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SqlConstraintError {
    Empty,
    UnknownKind,
    Identifier(SqlIdentifierError),
}

impl fmt::Display for SqlConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL constraint label cannot be empty"),
            Self::UnknownKind => formatter.write_str("unknown SQL constraint kind"),
            Self::Identifier(error) => {
                write!(formatter, "invalid SQL constraint identifier: {error}")
            },
        }
    }
}

impl Error for SqlConstraintError {}

fn normalized_constraint(input: &str) -> Result<String, SqlConstraintError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlConstraintError::Empty);
    }
    Ok(trimmed
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::{SqlConstraint, SqlConstraintError, SqlConstraintKind, SqlConstraintName};

    #[test]
    fn parses_and_renders_constraints() -> Result<(), SqlConstraintError> {
        assert_eq!(
            "primary key".parse::<SqlConstraintKind>()?,
            SqlConstraintKind::PrimaryKey
        );
        let constraint = SqlConstraint::new(SqlConstraintKind::Unique)
            .with_name(SqlConstraintName::new("users_email_key")?);
        assert_eq!(constraint.to_string(), "CONSTRAINT users_email_key UNIQUE");
        Ok(())
    }
}
