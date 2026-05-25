#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// SQL comparison operators.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlComparisonOperator {
    #[default]
    Equal,
    NotEqual,
    NotEqualBang,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

impl SqlComparisonOperator {
    /// Returns the stable operator label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "=",
            Self::NotEqual => "<>",
            Self::NotEqualBang => "!=",
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
        }
    }
}

impl fmt::Display for SqlComparisonOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// SQL logical operators.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlLogicalOperator {
    #[default]
    And,
    Or,
    Not,
}

impl SqlLogicalOperator {
    /// Returns the stable operator label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
        }
    }
}

impl fmt::Display for SqlLogicalOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// SQL null-check operators.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlNullOperator {
    #[default]
    IsNull,
    IsNotNull,
}

impl SqlNullOperator {
    /// Returns the stable operator label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsNull => "IS NULL",
            Self::IsNotNull => "IS NOT NULL",
        }
    }
}

impl fmt::Display for SqlNullOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// SQL pattern and membership operators.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlPatternOperator {
    #[default]
    Like,
    In,
}

impl SqlPatternOperator {
    /// Returns the stable operator label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Like => "LIKE",
            Self::In => "IN",
        }
    }
}

impl fmt::Display for SqlPatternOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Common SQL operators.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlOperator {
    Comparison(SqlComparisonOperator),
    Logical(SqlLogicalOperator),
    Null(SqlNullOperator),
    Pattern(SqlPatternOperator),
}

impl SqlOperator {
    /// Returns the stable operator label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comparison(operator) => operator.as_str(),
            Self::Logical(operator) => operator.as_str(),
            Self::Null(operator) => operator.as_str(),
            Self::Pattern(operator) => operator.as_str(),
        }
    }
}

impl fmt::Display for SqlOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlOperator {
    type Err = SqlOperatorParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_operator(input)?.as_str() {
            "=" => Ok(Self::Comparison(SqlComparisonOperator::Equal)),
            "<>" => Ok(Self::Comparison(SqlComparisonOperator::NotEqual)),
            "!=" => Ok(Self::Comparison(SqlComparisonOperator::NotEqualBang)),
            "<" => Ok(Self::Comparison(SqlComparisonOperator::Less)),
            "<=" => Ok(Self::Comparison(SqlComparisonOperator::LessOrEqual)),
            ">" => Ok(Self::Comparison(SqlComparisonOperator::Greater)),
            ">=" => Ok(Self::Comparison(SqlComparisonOperator::GreaterOrEqual)),
            "AND" => Ok(Self::Logical(SqlLogicalOperator::And)),
            "OR" => Ok(Self::Logical(SqlLogicalOperator::Or)),
            "NOT" => Ok(Self::Logical(SqlLogicalOperator::Not)),
            "LIKE" => Ok(Self::Pattern(SqlPatternOperator::Like)),
            "IN" => Ok(Self::Pattern(SqlPatternOperator::In)),
            "IS NULL" => Ok(Self::Null(SqlNullOperator::IsNull)),
            "IS NOT NULL" => Ok(Self::Null(SqlNullOperator::IsNotNull)),
            _ => Err(SqlOperatorParseError::Unknown),
        }
    }
}

impl TryFrom<&str> for SqlOperator {
    type Error = SqlOperatorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Error returned when parsing SQL operators fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlOperatorParseError {
    Empty,
    Unknown,
}

impl fmt::Display for SqlOperatorParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL operator cannot be empty"),
            Self::Unknown => formatter.write_str("unknown SQL operator"),
        }
    }
}

impl Error for SqlOperatorParseError {}

fn normalized_operator(input: &str) -> Result<String, SqlOperatorParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlOperatorParseError::Empty);
    }
    Ok(trimmed
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::{SqlComparisonOperator, SqlOperator, SqlOperatorParseError};

    #[test]
    fn parses_operator_labels() -> Result<(), SqlOperatorParseError> {
        assert_eq!("<=".parse::<SqlOperator>()?.to_string(), "<=");
        assert_eq!(
            "is not null".parse::<SqlOperator>()?.to_string(),
            "IS NOT NULL"
        );
        assert_eq!(SqlComparisonOperator::NotEqualBang.to_string(), "!=");
        Ok(())
    }

    #[test]
    fn rejects_unknown_operators() {
        assert_eq!("".parse::<SqlOperator>(), Err(SqlOperatorParseError::Empty));
        assert_eq!(
            "~~".parse::<SqlOperator>(),
            Err(SqlOperatorParseError::Unknown)
        );
    }
}
