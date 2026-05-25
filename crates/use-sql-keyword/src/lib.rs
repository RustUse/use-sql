#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common SQL keywords included by `use-sql-keyword`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlKeyword {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Alter,
    Drop,
    Table,
    View,
    Index,
    Where,
    From,
    Join,
    Group,
    Order,
    Limit,
    Offset,
    Returning,
    Primary,
    Foreign,
    Key,
    Unique,
    Not,
    Null,
    Check,
    Default,
}

impl SqlKeyword {
    /// Returns the uppercase SQL keyword label.
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
            Self::Table => "TABLE",
            Self::View => "VIEW",
            Self::Index => "INDEX",
            Self::Where => "WHERE",
            Self::From => "FROM",
            Self::Join => "JOIN",
            Self::Group => "GROUP",
            Self::Order => "ORDER",
            Self::Limit => "LIMIT",
            Self::Offset => "OFFSET",
            Self::Returning => "RETURNING",
            Self::Primary => "PRIMARY",
            Self::Foreign => "FOREIGN",
            Self::Key => "KEY",
            Self::Unique => "UNIQUE",
            Self::Not => "NOT",
            Self::Null => "NULL",
            Self::Check => "CHECK",
            Self::Default => "DEFAULT",
        }
    }

    /// Returns a broad keyword category.
    #[must_use]
    pub const fn kind(self) -> SqlKeywordKind {
        match self {
            Self::Select => SqlKeywordKind::DataQuery,
            Self::Insert | Self::Update | Self::Delete => SqlKeywordKind::DataMutation,
            Self::Create | Self::Alter | Self::Drop => SqlKeywordKind::Definition,
            Self::Table | Self::View | Self::Index => SqlKeywordKind::Object,
            Self::Where
            | Self::From
            | Self::Join
            | Self::Group
            | Self::Order
            | Self::Limit
            | Self::Offset
            | Self::Returning => SqlKeywordKind::Clause,
            Self::Primary | Self::Foreign | Self::Key | Self::Unique | Self::Check => {
                SqlKeywordKind::Constraint
            },
            Self::Not | Self::Null | Self::Default => SqlKeywordKind::Modifier,
        }
    }
}

impl fmt::Display for SqlKeyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlKeyword {
    type Err = SqlKeywordParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_word(input)?.as_str() {
            "SELECT" => Ok(Self::Select),
            "INSERT" => Ok(Self::Insert),
            "UPDATE" => Ok(Self::Update),
            "DELETE" => Ok(Self::Delete),
            "CREATE" => Ok(Self::Create),
            "ALTER" => Ok(Self::Alter),
            "DROP" => Ok(Self::Drop),
            "TABLE" => Ok(Self::Table),
            "VIEW" => Ok(Self::View),
            "INDEX" => Ok(Self::Index),
            "WHERE" => Ok(Self::Where),
            "FROM" => Ok(Self::From),
            "JOIN" => Ok(Self::Join),
            "GROUP" => Ok(Self::Group),
            "ORDER" => Ok(Self::Order),
            "LIMIT" => Ok(Self::Limit),
            "OFFSET" => Ok(Self::Offset),
            "RETURNING" => Ok(Self::Returning),
            "PRIMARY" => Ok(Self::Primary),
            "FOREIGN" => Ok(Self::Foreign),
            "KEY" => Ok(Self::Key),
            "UNIQUE" => Ok(Self::Unique),
            "NOT" => Ok(Self::Not),
            "NULL" => Ok(Self::Null),
            "CHECK" => Ok(Self::Check),
            "DEFAULT" => Ok(Self::Default),
            _ => Err(SqlKeywordParseError::Unknown),
        }
    }
}

impl TryFrom<&str> for SqlKeyword {
    type Error = SqlKeywordParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Broad categories for common SQL keywords.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlKeywordKind {
    #[default]
    DataQuery,
    DataMutation,
    Definition,
    Object,
    Clause,
    Constraint,
    Modifier,
}

/// Error returned when parsing a SQL keyword fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlKeywordParseError {
    Empty,
    Unknown,
}

impl fmt::Display for SqlKeywordParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL keyword cannot be empty"),
            Self::Unknown => formatter.write_str("unknown SQL keyword"),
        }
    }
}

impl Error for SqlKeywordParseError {}

/// Returns whether `input` is one of the common keywords in this crate.
#[must_use]
pub fn is_common_keyword(input: &str) -> bool {
    input.parse::<SqlKeyword>().is_ok()
}

/// Returns whether `input` is reserved-like for conservative SQL helper purposes.
#[must_use]
pub fn is_reserved_like(input: &str) -> bool {
    is_common_keyword(input)
}

fn normalized_word(input: &str) -> Result<String, SqlKeywordParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err(SqlKeywordParseError::Empty)
    } else {
        Ok(trimmed.to_ascii_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{SqlKeyword, SqlKeywordKind, SqlKeywordParseError, is_common_keyword};

    #[test]
    fn parses_common_keywords() -> Result<(), SqlKeywordParseError> {
        let keyword: SqlKeyword = "select".parse()?;
        assert_eq!(keyword, SqlKeyword::Select);
        assert_eq!(keyword.kind(), SqlKeywordKind::DataQuery);
        assert_eq!(keyword.to_string(), "SELECT");
        assert!(is_common_keyword("where"));
        Ok(())
    }

    #[test]
    fn rejects_unknown_keywords() {
        assert_eq!("".parse::<SqlKeyword>(), Err(SqlKeywordParseError::Empty));
        assert_eq!(
            "UPSERT".parse::<SqlKeyword>(),
            Err(SqlKeywordParseError::Unknown)
        );
    }
}
