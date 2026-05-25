#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Common SQL clause labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlClauseKind {
    #[default]
    Select,
    From,
    Where,
    GroupBy,
    Having,
    OrderBy,
    Limit,
    Offset,
    Returning,
}

impl SqlClauseKind {
    /// Returns the stable clause label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Select => "SELECT",
            Self::From => "FROM",
            Self::Where => "WHERE",
            Self::GroupBy => "GROUP BY",
            Self::Having => "HAVING",
            Self::OrderBy => "ORDER BY",
            Self::Limit => "LIMIT",
            Self::Offset => "OFFSET",
            Self::Returning => "RETURNING",
        }
    }

    /// Returns the common select-statement clause ordinal.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::Select => 10,
            Self::From => 20,
            Self::Where => 30,
            Self::GroupBy => 40,
            Self::Having => 50,
            Self::OrderBy => 60,
            Self::Limit => 70,
            Self::Offset => 80,
            Self::Returning => 90,
        }
    }

    /// Returns whether `self` commonly appears before `other`.
    #[must_use]
    pub const fn comes_before(self, other: Self) -> bool {
        self.ordinal() < other.ordinal()
    }

    /// Returns whether `self` commonly appears after `other`.
    #[must_use]
    pub const fn comes_after(self, other: Self) -> bool {
        self.ordinal() > other.ordinal()
    }
}

impl fmt::Display for SqlClauseKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlClauseKind {
    type Err = SqlClauseParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_clause(input)?.as_str() {
            "SELECT" => Ok(Self::Select),
            "FROM" => Ok(Self::From),
            "WHERE" => Ok(Self::Where),
            "GROUP BY" | "GROUP" => Ok(Self::GroupBy),
            "HAVING" => Ok(Self::Having),
            "ORDER BY" | "ORDER" => Ok(Self::OrderBy),
            "LIMIT" => Ok(Self::Limit),
            "OFFSET" => Ok(Self::Offset),
            "RETURNING" => Ok(Self::Returning),
            _ => Err(SqlClauseParseError::Unknown),
        }
    }
}

/// A SQL clause label with optional raw text metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlClause {
    kind: SqlClauseKind,
    text: Option<String>,
}

impl SqlClause {
    /// Creates a clause label.
    #[must_use]
    pub const fn new(kind: SqlClauseKind) -> Self {
        Self { kind, text: None }
    }

    /// Attaches raw clause text metadata.
    #[must_use]
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Returns the clause kind.
    #[must_use]
    pub const fn kind(&self) -> SqlClauseKind {
        self.kind
    }

    /// Returns optional raw clause text metadata.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

impl fmt::Display for SqlClause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.kind.as_str())
    }
}

/// Helper type for common clause ordering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlClauseOrder;

impl SqlClauseOrder {
    /// Sorts clause kinds by their common select-statement order.
    #[must_use]
    pub fn sort_kinds(mut kinds: Vec<SqlClauseKind>) -> Vec<SqlClauseKind> {
        kinds.sort_by_key(|kind| kind.ordinal());
        kinds
    }
}

/// Error returned when parsing clause labels fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlClauseParseError {
    Empty,
    Unknown,
}

impl fmt::Display for SqlClauseParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL clause label cannot be empty"),
            Self::Unknown => formatter.write_str("unknown SQL clause label"),
        }
    }
}

impl Error for SqlClauseParseError {}

fn normalized_clause(input: &str) -> Result<String, SqlClauseParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlClauseParseError::Empty);
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
    use super::{SqlClauseKind, SqlClauseOrder, SqlClauseParseError};

    #[test]
    fn parses_clause_labels() -> Result<(), SqlClauseParseError> {
        assert_eq!("group by".parse::<SqlClauseKind>()?, SqlClauseKind::GroupBy);
        assert!(SqlClauseKind::Where.comes_after(SqlClauseKind::From));
        Ok(())
    }

    #[test]
    fn sorts_clause_kinds() {
        let sorted = SqlClauseOrder::sort_kinds(vec![SqlClauseKind::Where, SqlClauseKind::Select]);
        assert_eq!(sorted, vec![SqlClauseKind::Select, SqlClauseKind::Where]);
    }
}
