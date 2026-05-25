#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for primitive SQL utility crates.

#[cfg(feature = "clause")]
pub use use_sql_clause as clause;

#[cfg(feature = "column")]
pub use use_sql_column as column;

#[cfg(feature = "constraint")]
pub use use_sql_constraint as constraint;

#[cfg(feature = "dialect")]
pub use use_sql_dialect as dialect;

#[cfg(feature = "expression")]
pub use use_sql_expression as expression;

#[cfg(feature = "ident")]
pub use use_sql_ident as ident;

#[cfg(feature = "keyword")]
pub use use_sql_keyword as keyword;

#[cfg(feature = "operator")]
pub use use_sql_operator as operator;

#[cfg(feature = "param")]
pub use use_sql_param as param;

#[cfg(feature = "query")]
pub use use_sql_query as query;

#[cfg(feature = "schema")]
pub use use_sql_schema as schema;

#[cfg(feature = "table")]
pub use use_sql_table as table;

#[cfg(feature = "ty")]
pub use use_sql_type as ty;

#[cfg(feature = "value")]
pub use use_sql_value as value;
