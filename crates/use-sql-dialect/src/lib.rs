#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Lightweight SQL dialect labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlDialect {
    #[default]
    Ansi,
    PostgreSql,
    SQLite,
    MySql,
    MariaDb,
    SqlServer,
    Oracle,
    DuckDb,
    BigQuery,
    Snowflake,
}

impl SqlDialect {
    /// Returns the stable lowercase dialect label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ansi => "ansi",
            Self::PostgreSql => "postgresql",
            Self::SQLite => "sqlite",
            Self::MySql => "mysql",
            Self::MariaDb => "mariadb",
            Self::SqlServer => "sql-server",
            Self::Oracle => "oracle",
            Self::DuckDb => "duckdb",
            Self::BigQuery => "bigquery",
            Self::Snowflake => "snowflake",
        }
    }

    /// Returns the broad dialect family.
    #[must_use]
    pub const fn family(self) -> SqlDialectFamily {
        match self {
            Self::Ansi => SqlDialectFamily::Standard,
            Self::PostgreSql => SqlDialectFamily::PostgreSql,
            Self::SQLite => SqlDialectFamily::SQLite,
            Self::MySql | Self::MariaDb => SqlDialectFamily::MySql,
            Self::SqlServer => SqlDialectFamily::SqlServer,
            Self::Oracle => SqlDialectFamily::Oracle,
            Self::DuckDb | Self::BigQuery | Self::Snowflake => SqlDialectFamily::Analytical,
        }
    }
}

impl fmt::Display for SqlDialect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SqlDialect {
    type Err = SqlDialectParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match normalized_label(input)?.as_str() {
            "ansi" | "sqlstandard" | "standard" => Ok(Self::Ansi),
            "postgres" | "postgresql" => Ok(Self::PostgreSql),
            "sqlite" | "sqlite3" => Ok(Self::SQLite),
            "mysql" => Ok(Self::MySql),
            "mariadb" => Ok(Self::MariaDb),
            "sqlserver" | "mssql" | "tsql" => Ok(Self::SqlServer),
            "oracle" => Ok(Self::Oracle),
            "duckdb" => Ok(Self::DuckDb),
            "bigquery" => Ok(Self::BigQuery),
            "snowflake" => Ok(Self::Snowflake),
            _ => Err(SqlDialectParseError::Unknown),
        }
    }
}

impl TryFrom<&str> for SqlDialect {
    type Error = SqlDialectParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Broad SQL dialect families.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlDialectFamily {
    #[default]
    Standard,
    PostgreSql,
    SQLite,
    MySql,
    SqlServer,
    Oracle,
    Analytical,
}

impl SqlDialectFamily {
    /// Returns the stable lowercase family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::PostgreSql => "postgresql",
            Self::SQLite => "sqlite",
            Self::MySql => "mysql",
            Self::SqlServer => "sql-server",
            Self::Oracle => "oracle",
            Self::Analytical => "analytical",
        }
    }
}

impl fmt::Display for SqlDialectFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error returned when parsing SQL dialect labels fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlDialectParseError {
    Empty,
    Unknown,
}

impl fmt::Display for SqlDialectParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("SQL dialect label cannot be empty"),
            Self::Unknown => formatter.write_str("unknown SQL dialect label"),
        }
    }
}

impl Error for SqlDialectParseError {}

fn normalized_label(input: &str) -> Result<String, SqlDialectParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SqlDialectParseError::Empty);
    }

    Ok(trimmed
        .chars()
        .filter(|character| !matches!(character, '-' | '_' | ' '))
        .collect::<String>()
        .to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{SqlDialect, SqlDialectFamily, SqlDialectParseError};

    #[test]
    fn parses_common_dialects() -> Result<(), SqlDialectParseError> {
        assert_eq!("postgres".parse::<SqlDialect>()?, SqlDialect::PostgreSql);
        assert_eq!("sql server".parse::<SqlDialect>()?, SqlDialect::SqlServer);
        assert_eq!(SqlDialect::Snowflake.family(), SqlDialectFamily::Analytical);
        Ok(())
    }

    #[test]
    fn rejects_unknown_dialects() {
        assert_eq!("".parse::<SqlDialect>(), Err(SqlDialectParseError::Empty));
        assert_eq!(
            "firebird".parse::<SqlDialect>(),
            Err(SqlDialectParseError::Unknown)
        );
    }
}
