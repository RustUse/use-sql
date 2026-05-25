#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

use use_sql_column::SqlColumnRef;
use use_sql_ident::SqlIdentifier;
use use_sql_operator::{
    SqlComparisonOperator, SqlLogicalOperator, SqlNullOperator, SqlOperator, SqlPatternOperator,
};
use use_sql_param::SqlParameter;
use use_sql_value::SqlValue;

/// Lightweight SQL expression container.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SqlExpression {
    kind: SqlExpressionKind,
}

impl SqlExpression {
    /// Creates an expression from an identifier.
    #[must_use]
    pub const fn identifier(identifier: SqlIdentifier) -> Self {
        Self {
            kind: SqlExpressionKind::Identifier(identifier),
        }
    }

    /// Creates an expression from a column reference.
    #[must_use]
    pub const fn column(column: SqlColumnRef) -> Self {
        Self {
            kind: SqlExpressionKind::Column(column),
        }
    }

    /// Creates an expression from a literal value.
    #[must_use]
    pub const fn value(value: SqlValue) -> Self {
        Self {
            kind: SqlExpressionKind::Value(value),
        }
    }

    /// Creates an expression from a parameter placeholder.
    #[must_use]
    pub const fn parameter(parameter: SqlParameter) -> Self {
        Self {
            kind: SqlExpressionKind::Parameter(parameter),
        }
    }

    /// Creates a simple binary operator expression.
    #[must_use]
    pub fn binary(left: Self, operator: SqlOperator, right: Self) -> Self {
        Self {
            kind: SqlExpressionKind::Operator {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
        }
    }

    /// Creates an expression from a predicate.
    #[must_use]
    pub fn predicate(predicate: SqlPredicate) -> Self {
        Self {
            kind: SqlExpressionKind::Predicate(Box::new(predicate)),
        }
    }

    /// Returns the expression kind.
    #[must_use]
    pub const fn kind(&self) -> &SqlExpressionKind {
        &self.kind
    }
}

impl fmt::Display for SqlExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            SqlExpressionKind::Identifier(identifier) => identifier.fmt(formatter),
            SqlExpressionKind::Column(column) => column.fmt(formatter),
            SqlExpressionKind::Value(value) => value.fmt(formatter),
            SqlExpressionKind::Parameter(parameter) => parameter.fmt(formatter),
            SqlExpressionKind::Operator {
                left,
                operator,
                right,
            } => write!(formatter, "({left} {operator} {right})"),
            SqlExpressionKind::Predicate(predicate) => predicate.fmt(formatter),
        }
    }
}

impl From<SqlIdentifier> for SqlExpression {
    fn from(value: SqlIdentifier) -> Self {
        Self::identifier(value)
    }
}

impl From<SqlColumnRef> for SqlExpression {
    fn from(value: SqlColumnRef) -> Self {
        Self::column(value)
    }
}

impl From<SqlValue> for SqlExpression {
    fn from(value: SqlValue) -> Self {
        Self::value(value)
    }
}

impl From<SqlParameter> for SqlExpression {
    fn from(value: SqlParameter) -> Self {
        Self::parameter(value)
    }
}

/// Lightweight SQL expression kinds.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlExpressionKind {
    Identifier(SqlIdentifier),
    Column(SqlColumnRef),
    Value(SqlValue),
    Parameter(SqlParameter),
    Operator {
        left: Box<SqlExpression>,
        operator: SqlOperator,
        right: Box<SqlExpression>,
    },
    Predicate(Box<SqlPredicate>),
}

/// Lightweight SQL predicate containers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SqlPredicate {
    Comparison {
        left: Box<SqlExpression>,
        operator: SqlComparisonOperator,
        right: Box<SqlExpression>,
    },
    Null {
        expression: Box<SqlExpression>,
        operator: SqlNullOperator,
    },
    Pattern {
        left: Box<SqlExpression>,
        operator: SqlPatternOperator,
        right: Box<SqlExpression>,
    },
    Logical {
        left: Box<Self>,
        operator: SqlLogicalOperator,
        right: Box<Self>,
    },
    Not(Box<Self>),
}

impl SqlPredicate {
    /// Creates a comparison predicate.
    #[must_use]
    pub fn comparison(
        left: SqlExpression,
        operator: SqlComparisonOperator,
        right: SqlExpression,
    ) -> Self {
        Self::Comparison {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    /// Creates a null-check predicate.
    #[must_use]
    pub fn null(expression: SqlExpression, operator: SqlNullOperator) -> Self {
        Self::Null {
            expression: Box::new(expression),
            operator,
        }
    }

    /// Creates a pattern or membership predicate.
    #[must_use]
    pub fn pattern(
        left: SqlExpression,
        operator: SqlPatternOperator,
        right: SqlExpression,
    ) -> Self {
        Self::Pattern {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    /// Creates a logical predicate.
    #[must_use]
    pub fn logical(left: Self, operator: SqlLogicalOperator, right: Self) -> Self {
        Self::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    /// Creates a negated predicate.
    #[must_use]
    pub fn negate(predicate: Self) -> Self {
        Self::Not(Box::new(predicate))
    }
}

impl fmt::Display for SqlPredicate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comparison {
                left,
                operator,
                right,
            } => write!(formatter, "{left} {operator} {right}"),
            Self::Null {
                expression,
                operator,
            } => write!(formatter, "{expression} {operator}"),
            Self::Pattern {
                left,
                operator,
                right,
            } => write!(formatter, "{left} {operator} {right}"),
            Self::Logical {
                left,
                operator,
                right,
            } => write!(formatter, "({left}) {operator} ({right})"),
            Self::Not(predicate) => write!(formatter, "NOT ({predicate})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SqlExpression, SqlPredicate};
    use use_sql_column::{SqlColumnName, SqlColumnRef};
    use use_sql_operator::{SqlComparisonOperator, SqlNullOperator};
    use use_sql_param::SqlParameter;
    use use_sql_value::SqlValue;

    #[test]
    fn renders_simple_expressions() -> Result<(), Box<dyn std::error::Error>> {
        let column = SqlExpression::column(SqlColumnRef::new(SqlColumnName::new("id")?));
        let parameter = SqlExpression::parameter("$1".parse::<SqlParameter>()?);
        let predicate = SqlPredicate::comparison(column, SqlComparisonOperator::Equal, parameter);

        assert_eq!(predicate.to_string(), "id = $1");
        assert_eq!(
            SqlExpression::value(SqlValue::string("Ada")).to_string(),
            "'Ada'"
        );
        Ok(())
    }

    #[test]
    fn renders_null_predicates() -> Result<(), Box<dyn std::error::Error>> {
        let column = SqlExpression::column(SqlColumnRef::new(SqlColumnName::new("deleted_at")?));
        let predicate = SqlPredicate::null(column, SqlNullOperator::IsNull);

        assert_eq!(predicate.to_string(), "deleted_at IS NULL");
        Ok(())
    }
}
