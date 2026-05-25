#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_sql_ident::{SqlIdentifier, SqlIdentifierError, is_valid_unquoted_ident};

/// SQL parameter placeholder styles.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlParameterStyle {
    #[default]
    PostgresIndexed,
    PositionalQuestionMark,
    NamedColon,
    NamedAtSign,
}

/// A one-based SQL parameter index.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlParameterIndex(u32);

impl SqlParameterIndex {
    /// Creates a one-based parameter index.
    ///
    /// # Errors
    ///
    /// Returns [`SqlParameterError::ZeroIndex`] when `index` is zero.
    pub const fn new(index: u32) -> Result<Self, SqlParameterError> {
        if index == 0 {
            Err(SqlParameterError::ZeroIndex)
        } else {
            Ok(Self(index))
        }
    }

    /// Returns the one-based parameter index.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SqlParameterIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A named SQL parameter identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlParameterName(SqlIdentifier);

impl SqlParameterName {
    /// Creates a named SQL parameter.
    ///
    /// # Errors
    ///
    /// Returns [`SqlParameterError`] when the parameter name is empty or not
    /// conservatively unquoted-identifier-shaped.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlParameterError> {
        let input = input.as_ref();
        if !is_valid_unquoted_ident(input) {
            return Err(SqlParameterError::InvalidName);
        }

        SqlIdentifier::new(input)
            .map(Self)
            .map_err(SqlParameterError::Identifier)
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SqlParameterName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlParameterName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SqlParameterName {
    type Err = SqlParameterError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

/// SQL parameter placeholders.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlParameter {
    PostgresIndexed(SqlParameterIndex),
    PositionalQuestionMark,
    NamedColon(SqlParameterName),
    NamedAtSign(SqlParameterName),
}

impl SqlParameter {
    /// Creates a PostgreSQL-style indexed parameter such as `$1`.
    ///
    /// # Errors
    ///
    /// Returns [`SqlParameterError::ZeroIndex`] when `index` is zero.
    pub const fn postgres_indexed(index: u32) -> Result<Self, SqlParameterError> {
        match SqlParameterIndex::new(index) {
            Ok(index) => Ok(Self::PostgresIndexed(index)),
            Err(error) => Err(error),
        }
    }

    /// Creates a positional question-mark parameter.
    #[must_use]
    pub const fn positional() -> Self {
        Self::PositionalQuestionMark
    }

    /// Creates a colon-prefixed named parameter.
    ///
    /// # Errors
    ///
    /// Returns [`SqlParameterError`] when name validation fails.
    pub fn named_colon(name: impl AsRef<str>) -> Result<Self, SqlParameterError> {
        SqlParameterName::new(name).map(Self::NamedColon)
    }

    /// Creates an at-sign-prefixed named parameter.
    ///
    /// # Errors
    ///
    /// Returns [`SqlParameterError`] when name validation fails.
    pub fn named_at(name: impl AsRef<str>) -> Result<Self, SqlParameterError> {
        SqlParameterName::new(name).map(Self::NamedAtSign)
    }

    /// Returns the placeholder style.
    #[must_use]
    pub const fn style(&self) -> SqlParameterStyle {
        match self {
            Self::PostgresIndexed(_) => SqlParameterStyle::PostgresIndexed,
            Self::PositionalQuestionMark => SqlParameterStyle::PositionalQuestionMark,
            Self::NamedColon(_) => SqlParameterStyle::NamedColon,
            Self::NamedAtSign(_) => SqlParameterStyle::NamedAtSign,
        }
    }
}

impl fmt::Display for SqlParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PostgresIndexed(index) => write!(formatter, "${index}"),
            Self::PositionalQuestionMark => formatter.write_str("?"),
            Self::NamedColon(name) => write!(formatter, ":{name}"),
            Self::NamedAtSign(name) => write!(formatter, "@{name}"),
        }
    }
}

impl FromStr for SqlParameter {
    type Err = SqlParameterError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(SqlParameterError::Empty);
        }
        if trimmed == "?" {
            return Ok(Self::positional());
        }
        if let Some(index) = trimmed.strip_prefix('$') {
            if index.is_empty() || !index.chars().all(|character| character.is_ascii_digit()) {
                return Err(SqlParameterError::InvalidIndexed);
            }
            let index = index
                .parse::<u32>()
                .map_err(|_| SqlParameterError::InvalidIndexed)?;
            return Self::postgres_indexed(index);
        }
        if let Some(name) = trimmed.strip_prefix(':') {
            return Self::named_colon(name);
        }
        if let Some(name) = trimmed.strip_prefix('@') {
            return Self::named_at(name);
        }
        Err(SqlParameterError::UnknownStyle)
    }
}

impl TryFrom<&str> for SqlParameter {
    type Error = SqlParameterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Error returned when SQL parameter placeholders are invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SqlParameterError {
    Empty,
    ZeroIndex,
    InvalidIndexed,
    InvalidName,
    UnknownStyle,
    Identifier(SqlIdentifierError),
}

impl fmt::Display for SqlParameterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL parameter placeholder cannot be empty"),
            Self::ZeroIndex => formatter.write_str("SQL parameter indexes are one-based"),
            Self::InvalidIndexed => formatter.write_str("invalid indexed SQL parameter"),
            Self::InvalidName => formatter.write_str("invalid SQL parameter name"),
            Self::UnknownStyle => formatter.write_str("unknown SQL parameter placeholder style"),
            Self::Identifier(error) => {
                write!(formatter, "invalid SQL parameter identifier: {error}")
            },
        }
    }
}

impl Error for SqlParameterError {}

#[cfg(test)]
mod tests {
    use super::{SqlParameter, SqlParameterError, SqlParameterStyle};

    #[test]
    fn parses_parameter_styles() -> Result<(), SqlParameterError> {
        assert_eq!("$1".parse::<SqlParameter>()?.to_string(), "$1");
        assert_eq!(
            "?".parse::<SqlParameter>()?.style(),
            SqlParameterStyle::PositionalQuestionMark
        );
        assert_eq!(":user_id".parse::<SqlParameter>()?.to_string(), ":user_id");
        assert_eq!("@user_id".parse::<SqlParameter>()?.to_string(), "@user_id");
        Ok(())
    }

    #[test]
    fn rejects_invalid_parameters() {
        assert_eq!(
            "$0".parse::<SqlParameter>(),
            Err(SqlParameterError::ZeroIndex)
        );
        assert_eq!(
            "$abc".parse::<SqlParameter>(),
            Err(SqlParameterError::InvalidIndexed)
        );
        assert_eq!(
            ":select".parse::<SqlParameter>(),
            Err(SqlParameterError::InvalidName)
        );
    }
}
