#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A validated SQL identifier segment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlIdentifier(String);

impl SqlIdentifier {
    /// Creates an identifier segment from conservative SQL identifier text.
    ///
    /// # Errors
    ///
    /// Returns [`SqlIdentifierError`] when the value is empty, contains a dot,
    /// or contains control characters.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlIdentifierError> {
        validate_identifier_text(input.as_ref()).map(|value| Self(value.to_owned()))
    }

    /// Returns the stored identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the stored text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SqlIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlIdentifier {
    type Err = SqlIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for SqlIdentifier {
    type Error = SqlIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A dot-qualified SQL name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlQualifiedName {
    parts: Vec<SqlIdentifier>,
}

impl SqlQualifiedName {
    /// Creates a qualified name from one or more identifier parts.
    ///
    /// # Errors
    ///
    /// Returns [`SqlIdentifierError::EmptyQualifiedName`] when `parts` is empty.
    pub fn new(parts: Vec<SqlIdentifier>) -> Result<Self, SqlIdentifierError> {
        if parts.is_empty() {
            return Err(SqlIdentifierError::EmptyQualifiedName);
        }

        Ok(Self { parts })
    }

    /// Parses a dot-qualified name using conservative dot splitting.
    ///
    /// # Errors
    ///
    /// Returns [`SqlIdentifierError`] when any segment is invalid.
    pub fn parse(input: &str) -> Result<Self, SqlIdentifierError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(SqlIdentifierError::EmptyQualifiedName);
        }

        let parts = trimmed
            .split('.')
            .map(SqlIdentifier::new)
            .collect::<Result<Vec<_>, _>>()?;
        Self::new(parts)
    }

    /// Returns the identifier parts.
    #[must_use]
    pub fn parts(&self) -> &[SqlIdentifier] {
        &self.parts
    }
}

impl fmt::Display for SqlQualifiedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = self.parts.iter();
        if let Some(first) = parts.next() {
            write!(formatter, "{first}")?;
        }
        for part in parts {
            write!(formatter, ".{part}")?;
        }
        Ok(())
    }
}

impl FromStr for SqlQualifiedName {
    type Err = SqlIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

impl TryFrom<&str> for SqlQualifiedName {
    type Error = SqlIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// A SQL alias identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlAlias(SqlIdentifier);

impl SqlAlias {
    /// Creates an alias from identifier text.
    ///
    /// # Errors
    ///
    /// Returns [`SqlIdentifierError`] when validation fails.
    pub fn new(input: impl AsRef<str>) -> Result<Self, SqlIdentifierError> {
        SqlIdentifier::new(input).map(Self)
    }

    /// Returns the alias as an identifier.
    #[must_use]
    pub const fn identifier(&self) -> &SqlIdentifier {
        &self.0
    }

    /// Returns the stored alias text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SqlAlias {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SqlAlias {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SqlAlias {
    type Err = SqlIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input)
    }
}

impl TryFrom<&str> for SqlAlias {
    type Error = SqlIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Error returned when SQL identifier text is rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlIdentifierError {
    /// The supplied value was empty after trimming.
    Empty,
    /// A single identifier segment cannot contain `.`.
    ContainsDot,
    /// A qualified name requires at least one segment.
    EmptyQualifiedName,
    /// The supplied value contained a control character.
    ControlCharacter {
        /// Byte index of the rejected character.
        index: usize,
        /// The rejected character.
        character: char,
    },
}

impl fmt::Display for SqlIdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL identifier cannot be empty"),
            Self::ContainsDot => formatter.write_str("SQL identifier segment cannot contain a dot"),
            Self::EmptyQualifiedName => formatter.write_str("SQL qualified name cannot be empty"),
            Self::ControlCharacter { index, character } => write!(
                formatter,
                "SQL identifier contains control character {character:?} at byte index {index}"
            ),
        }
    }
}

impl Error for SqlIdentifierError {}

/// Returns `true` when `input` is conservatively valid as an unquoted SQL identifier.
#[must_use]
pub fn is_valid_unquoted_ident(input: &str) -> bool {
    validate_unquoted_ident(input).is_ok()
}

/// Returns `true` when an identifier should be quoted for conservative SQL rendering.
#[must_use]
pub fn needs_quoting(input: &str) -> bool {
    !is_valid_unquoted_ident(input)
}

/// Quotes an identifier with SQL double quotes, doubling embedded double quotes.
#[must_use]
pub fn quote_ident(input: &str) -> String {
    let trimmed = input.trim();
    let mut quoted = String::with_capacity(trimmed.len() + 2);
    quoted.push('"');
    for character in trimmed.chars() {
        if character == '"' {
            quoted.push('"');
        }
        quoted.push(character);
    }
    quoted.push('"');
    quoted
}

/// Normalizes an identifier for simple display-oriented comparisons.
#[must_use]
pub fn normalize_ident(input: &str) -> String {
    let trimmed = input.trim();
    if is_valid_unquoted_ident(trimmed) {
        trimmed.to_ascii_lowercase()
    } else {
        quote_ident(trimmed)
    }
}

fn validate_identifier_text(input: &str) -> Result<&str, SqlIdentifierError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlIdentifierError::Empty);
    }
    if trimmed.contains('.') {
        return Err(SqlIdentifierError::ContainsDot);
    }
    if let Some((index, character)) = trimmed
        .char_indices()
        .find(|(_, character)| character.is_control())
    {
        return Err(SqlIdentifierError::ControlCharacter { index, character });
    }
    Ok(trimmed)
}

fn validate_unquoted_ident(input: &str) -> Result<(), SqlIdentifierError> {
    let trimmed = validate_identifier_text(input)?;
    let mut characters = trimmed.chars();
    let Some(first) = characters.next() else {
        return Err(SqlIdentifierError::Empty);
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(SqlIdentifierError::Empty);
    }
    if !characters.all(|character| character == '_' || character.is_ascii_alphanumeric()) {
        return Err(SqlIdentifierError::Empty);
    }
    if is_reserved_like(trimmed) {
        return Err(SqlIdentifierError::Empty);
    }
    Ok(())
}

fn is_reserved_like(input: &str) -> bool {
    matches!(
        input.trim().to_ascii_uppercase().as_str(),
        "SELECT"
            | "INSERT"
            | "UPDATE"
            | "DELETE"
            | "CREATE"
            | "ALTER"
            | "DROP"
            | "TABLE"
            | "VIEW"
            | "INDEX"
            | "WHERE"
            | "FROM"
            | "JOIN"
            | "GROUP"
            | "ORDER"
            | "LIMIT"
            | "OFFSET"
            | "RETURNING"
            | "PRIMARY"
            | "FOREIGN"
            | "KEY"
            | "UNIQUE"
            | "NOT"
            | "NULL"
            | "CHECK"
            | "DEFAULT"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        SqlIdentifier, SqlIdentifierError, SqlQualifiedName, is_valid_unquoted_ident,
        needs_quoting, normalize_ident, quote_ident,
    };

    #[test]
    fn validates_identifier_text() -> Result<(), SqlIdentifierError> {
        let identifier = SqlIdentifier::new(" users ")?;
        assert_eq!(identifier.as_str(), "users");
        assert_eq!(SqlIdentifier::new(""), Err(SqlIdentifierError::Empty));
        assert_eq!(
            SqlIdentifier::new("public.users"),
            Err(SqlIdentifierError::ContainsDot)
        );
        Ok(())
    }

    #[test]
    fn checks_unquoted_identifiers() {
        assert!(is_valid_unquoted_ident("users_1"));
        assert!(!is_valid_unquoted_ident("1users"));
        assert!(!is_valid_unquoted_ident("select"));
        assert!(needs_quoting("order items"));
    }

    #[test]
    fn quotes_and_normalizes_identifiers() {
        assert_eq!(quote_ident("user\"name"), "\"user\"\"name\"");
        assert_eq!(normalize_ident("Users"), "users");
        assert_eq!(normalize_ident("select"), "\"select\"");
    }

    #[test]
    fn parses_qualified_names() -> Result<(), SqlIdentifierError> {
        let qualified = SqlQualifiedName::parse("public.users")?;
        assert_eq!(qualified.parts().len(), 2);
        assert_eq!(qualified.to_string(), "public.users");
        assert!(SqlQualifiedName::parse("public.").is_err());
        Ok(())
    }
}
