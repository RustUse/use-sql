#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// SQL null literal marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlNull;

impl fmt::Display for SqlNull {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("NULL")
    }
}

/// SQL string literal text.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlStringLiteral(String);

impl SqlStringLiteral {
    /// Creates a string literal from text.
    #[must_use]
    pub fn new(input: impl Into<String>) -> Self {
        Self(input.into())
    }

    /// Returns the unescaped literal text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SqlStringLiteral {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlStringLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("'")?;
        for character in self.0.chars() {
            if character == '\'' {
                formatter.write_str("'")?;
            }
            write!(formatter, "{character}")?;
        }
        formatter.write_str("'")
    }
}

/// SQL number literal text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlNumberLiteral(String);

impl SqlNumberLiteral {
    /// Creates a conservatively validated number literal.
    ///
    /// # Errors
    ///
    /// Returns [`SqlValueError`] when the number is empty or not a finite numeric literal.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlValueError> {
        let trimmed = input.as_ref().trim();
        if trimmed.is_empty() {
            return Err(SqlValueError::EmptyNumber);
        }
        if !trimmed.chars().any(|character| character.is_ascii_digit()) {
            return Err(SqlValueError::InvalidNumber);
        }
        let value = trimmed
            .parse::<f64>()
            .map_err(|_| SqlValueError::InvalidNumber)?;
        if !value.is_finite() {
            return Err(SqlValueError::InvalidNumber);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the stored number literal text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SqlNumberLiteral {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlNumberLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlNumberLiteral {
    type Err = SqlValueError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for SqlNumberLiteral {
    type Error = SqlValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// SQL boolean literal.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlBooleanLiteral(bool);

impl SqlBooleanLiteral {
    /// Creates a boolean literal.
    #[must_use]
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the stored boolean value.
    #[must_use]
    pub const fn value(self) -> bool {
        self.0
    }
}

impl fmt::Display for SqlBooleanLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(if self.0 { "TRUE" } else { "FALSE" })
    }
}

/// Simple SQL literal/value primitives.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlValue {
    Null(SqlNull),
    String(SqlStringLiteral),
    Number(SqlNumberLiteral),
    Boolean(SqlBooleanLiteral),
}

impl SqlValue {
    /// Returns a null value.
    #[must_use]
    pub const fn null() -> Self {
        Self::Null(SqlNull)
    }

    /// Returns a string literal value.
    #[must_use]
    pub fn string(input: impl Into<String>) -> Self {
        Self::String(SqlStringLiteral::new(input))
    }

    /// Returns a number literal value.
    ///
    /// # Errors
    ///
    /// Returns [`SqlValueError`] when number validation fails.
    pub fn number(input: impl AsRef<str>) -> Result<Self, SqlValueError> {
        SqlNumberLiteral::new(input).map(Self::Number)
    }

    /// Returns a boolean literal value.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self::Boolean(SqlBooleanLiteral::new(value))
    }
}

impl Default for SqlValue {
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Display for SqlValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null(value) => value.fmt(formatter),
            Self::String(value) => value.fmt(formatter),
            Self::Number(value) => value.fmt(formatter),
            Self::Boolean(value) => value.fmt(formatter),
        }
    }
}

/// Error returned when SQL literal values are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlValueError {
    EmptyNumber,
    InvalidNumber,
}

impl fmt::Display for SqlValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNumber => formatter.write_str("SQL number literal cannot be empty"),
            Self::InvalidNumber => formatter.write_str("invalid SQL number literal"),
        }
    }
}

impl Error for SqlValueError {}

#[cfg(test)]
mod tests {
    use super::{SqlBooleanLiteral, SqlNumberLiteral, SqlValue, SqlValueError};

    #[test]
    fn renders_simple_literals() -> Result<(), SqlValueError> {
        assert_eq!(SqlValue::null().to_string(), "NULL");
        assert_eq!(SqlValue::string("Ada's").to_string(), "'Ada''s'");
        assert_eq!(SqlValue::number("42.5")?.to_string(), "42.5");
        assert_eq!(SqlBooleanLiteral::new(true).to_string(), "TRUE");
        Ok(())
    }

    #[test]
    fn validates_number_literals() {
        assert_eq!(SqlNumberLiteral::new(""), Err(SqlValueError::EmptyNumber));
        assert_eq!(
            SqlNumberLiteral::new("NaN"),
            Err(SqlValueError::InvalidNumber)
        );
        assert_eq!(
            SqlNumberLiteral::new("1e999"),
            Err(SqlValueError::InvalidNumber)
        );
    }
}
