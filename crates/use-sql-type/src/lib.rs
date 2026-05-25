#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A SQL type name label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlTypeName(String);

impl SqlTypeName {
    /// Creates a SQL type name label.
    ///
    /// # Errors
    ///
    /// Returns [`SqlTypeError`] when the label is empty or contains control characters.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlTypeError> {
        validate_type_label(input.as_ref()).map(|value| Self(value.to_owned()))
    }

    /// Creates a canonical type name from a scalar type.
    #[must_use]
    pub fn from_scalar(scalar: SqlScalarType) -> Self {
        Self(scalar.as_str().to_owned())
    }

    /// Returns the stored type name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SqlTypeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlTypeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlTypeName {
    type Err = SqlTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for SqlTypeName {
    type Error = SqlTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Common scalar SQL type labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlScalarType {
    #[default]
    Text,
    Integer,
    BigInt,
    Boolean,
    Decimal,
    Float,
    Date,
    Time,
    Timestamp,
    Json,
    Uuid,
    Binary,
}

impl SqlScalarType {
    /// Returns the stable lowercase scalar type label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Integer => "integer",
            Self::BigInt => "bigint",
            Self::Boolean => "boolean",
            Self::Decimal => "decimal",
            Self::Float => "float",
            Self::Date => "date",
            Self::Time => "time",
            Self::Timestamp => "timestamp",
            Self::Json => "json",
            Self::Uuid => "uuid",
            Self::Binary => "binary",
        }
    }
}

impl fmt::Display for SqlScalarType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlScalarType {
    type Err = SqlTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_type_label(input)?.as_str() {
            "text" | "string" | "varchar" | "character varying" | "char" => Ok(Self::Text),
            "int" | "integer" => Ok(Self::Integer),
            "bigint" | "big int" => Ok(Self::BigInt),
            "bool" | "boolean" => Ok(Self::Boolean),
            "decimal" | "numeric" => Ok(Self::Decimal),
            "float" | "real" | "double" | "double precision" => Ok(Self::Float),
            "date" => Ok(Self::Date),
            "time" => Ok(Self::Time),
            "timestamp" | "datetime" => Ok(Self::Timestamp),
            "json" | "jsonb" => Ok(Self::Json),
            "uuid" => Ok(Self::Uuid),
            "binary" | "blob" | "bytea" => Ok(Self::Binary),
            _ => Err(SqlTypeError::UnknownScalar),
        }
    }
}

impl TryFrom<&str> for SqlScalarType {
    type Error = SqlTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Lightweight SQL type modifier labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlTypeModifier {
    Array,
    Nullable,
    NotNull,
    Precision { precision: u16, scale: Option<u16> },
}

impl fmt::Display for SqlTypeModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array => formatter.write_str("ARRAY"),
            Self::Nullable => formatter.write_str("NULL"),
            Self::NotNull => formatter.write_str("NOT NULL"),
            Self::Precision { precision, scale } => {
                if let Some(scale) = scale {
                    write!(formatter, "({precision}, {scale})")
                } else {
                    write!(formatter, "({precision})")
                }
            },
        }
    }
}

/// Error returned when SQL type labels are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlTypeError {
    Empty,
    ControlCharacter,
    UnknownScalar,
}

impl fmt::Display for SqlTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL type label cannot be empty"),
            Self::ControlCharacter => {
                formatter.write_str("SQL type label cannot contain control characters")
            },
            Self::UnknownScalar => formatter.write_str("unknown SQL scalar type label"),
        }
    }
}

impl Error for SqlTypeError {}

fn validate_type_label(input: &str) -> Result<&str, SqlTypeError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlTypeError::Empty);
    }
    if trimmed.chars().any(char::is_control) {
        return Err(SqlTypeError::ControlCharacter);
    }
    Ok(trimmed)
}

fn normalized_type_label(input: &str) -> Result<String, SqlTypeError> {
    let trimmed = validate_type_label(input)?;
    Ok(trimmed
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{SqlScalarType, SqlTypeError, SqlTypeModifier, SqlTypeName};

    #[test]
    fn parses_common_scalar_types() -> Result<(), SqlTypeError> {
        assert_eq!("varchar".parse::<SqlScalarType>()?, SqlScalarType::Text);
        assert_eq!("numeric".parse::<SqlScalarType>()?, SqlScalarType::Decimal);
        assert_eq!("blob".parse::<SqlScalarType>()?, SqlScalarType::Binary);
        Ok(())
    }

    #[test]
    fn validates_type_names() -> Result<(), SqlTypeError> {
        let name = SqlTypeName::new(" NUMERIC ")?;
        assert_eq!(name.as_str(), "NUMERIC");
        assert_eq!(
            SqlTypeName::from_scalar(SqlScalarType::Uuid).to_string(),
            "uuid"
        );
        assert_eq!(SqlTypeName::new(""), Err(SqlTypeError::Empty));
        Ok(())
    }

    #[test]
    fn renders_modifiers() {
        assert_eq!(SqlTypeModifier::NotNull.to_string(), "NOT NULL");
        assert_eq!(
            SqlTypeModifier::Precision {
                precision: 10,
                scale: Some(2)
            }
            .to_string(),
            "(10, 2)"
        );
    }
}
