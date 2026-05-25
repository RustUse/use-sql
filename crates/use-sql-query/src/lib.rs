#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common SQL query kind labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlQueryKind {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Alter,
    Drop,
    Truncate,
    Merge,
    Explain,
    Vacuum,
    #[default]
    Unknown,
}

impl SqlQueryKind {
    /// Returns the stable uppercase query kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Select => "SELECT",
            Self::Insert => "INSERT",
            Self::Update => "UPDATE",
            Self::Delete => "DELETE",
            Self::Create => "CREATE",
            Self::Alter => "ALTER",
            Self::Drop => "DROP",
            Self::Truncate => "TRUNCATE",
            Self::Merge => "MERGE",
            Self::Explain => "EXPLAIN",
            Self::Vacuum => "VACUUM",
            Self::Unknown => "UNKNOWN",
        }
    }

    /// Returns whether the query kind is conservatively read-only.
    #[must_use]
    pub const fn is_read(self) -> bool {
        matches!(self, Self::Select | Self::Explain)
    }

    /// Returns whether the query kind commonly mutates data or schema.
    #[must_use]
    pub const fn is_write(self) -> bool {
        matches!(
            self,
            Self::Insert
                | Self::Update
                | Self::Delete
                | Self::Create
                | Self::Alter
                | Self::Drop
                | Self::Truncate
                | Self::Merge
                | Self::Vacuum
        )
    }

    /// Returns whether the query kind commonly changes schema.
    #[must_use]
    pub const fn is_schema_change(self) -> bool {
        matches!(
            self,
            Self::Create | Self::Alter | Self::Drop | Self::Truncate
        )
    }

    /// Returns whether the query kind is conservatively destructive.
    #[must_use]
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Delete | Self::Drop | Self::Truncate)
    }

    /// Returns a broad query intent label.
    #[must_use]
    pub const fn intent(self) -> SqlQueryIntent {
        if self.is_read() {
            SqlQueryIntent::Read
        } else if self.is_schema_change() {
            SqlQueryIntent::SchemaChange
        } else if matches!(self, Self::Vacuum) {
            SqlQueryIntent::Maintenance
        } else if self.is_write() {
            SqlQueryIntent::Write
        } else {
            SqlQueryIntent::Unknown
        }
    }

    /// Returns a broad safety label.
    #[must_use]
    pub const fn safety(self) -> SqlQuerySafety {
        if self.is_destructive() {
            SqlQuerySafety::Destructive
        } else if self.is_write() {
            SqlQuerySafety::Mutating
        } else if self.is_read() {
            SqlQuerySafety::ReadOnly
        } else {
            SqlQuerySafety::Unknown
        }
    }

    /// Classifies the first SQL-looking word without full parsing.
    #[must_use]
    pub fn classify(input: &str) -> Self {
        input
            .split_whitespace()
            .next()
            .and_then(|word| word.parse().ok())
            .unwrap_or(Self::Unknown)
    }
}

impl fmt::Display for SqlQueryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlQueryKind {
    type Err = SqlQueryKindParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_kind(input)?.as_str() {
            "SELECT" => Ok(Self::Select),
            "INSERT" => Ok(Self::Insert),
            "UPDATE" => Ok(Self::Update),
            "DELETE" => Ok(Self::Delete),
            "CREATE" => Ok(Self::Create),
            "ALTER" => Ok(Self::Alter),
            "DROP" => Ok(Self::Drop),
            "TRUNCATE" => Ok(Self::Truncate),
            "MERGE" => Ok(Self::Merge),
            "EXPLAIN" => Ok(Self::Explain),
            "VACUUM" => Ok(Self::Vacuum),
            _ => Ok(Self::Unknown),
        }
    }
}

/// Broad query intent labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlQueryIntent {
    Read,
    Write,
    SchemaChange,
    Maintenance,
    #[default]
    Unknown,
}

/// Broad query safety labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlQuerySafety {
    ReadOnly,
    Mutating,
    Destructive,
    #[default]
    Unknown,
}

/// Error returned when parsing SQL query kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlQueryKindParseError {
    Empty,
}

impl fmt::Display for SqlQueryKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL query kind cannot be empty"),
        }
    }
}

impl Error for SqlQueryKindParseError {}

fn normalized_kind(input: &str) -> Result<String, SqlQueryKindParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(SqlQueryKindParseError::Empty)
    } else {
        Ok(trimmed.to_ascii_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{SqlQueryIntent, SqlQueryKind, SqlQueryKindParseError, SqlQuerySafety};

    #[test]
    fn classifies_query_kinds() -> Result<(), SqlQueryKindParseError> {
        let delete: SqlQueryKind = "delete".parse()?;
        assert!(SqlQueryKind::Select.is_read());
        assert!(delete.is_write());
        assert!(delete.is_destructive());
        assert_eq!(SqlQueryKind::Create.intent(), SqlQueryIntent::SchemaChange);
        assert_eq!(SqlQueryKind::Drop.safety(), SqlQuerySafety::Destructive);
        Ok(())
    }

    #[test]
    fn classifies_first_token_conservatively() {
        assert_eq!(
            SqlQueryKind::classify(" select * from users"),
            SqlQueryKind::Select
        );
        assert_eq!(
            SqlQueryKind::classify("with users as (...)"),
            SqlQueryKind::Unknown
        );
    }
}
