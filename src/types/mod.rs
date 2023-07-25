use crate::{traits::{statement::Statement, expression::Expression}, parser::Parser};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type StatementResponse = Result<Box<dyn Statement>>;
pub type ExpressionResponse = Result<Box<dyn Expression>>;
pub type PrefixParseFn<'a> = fn(parser: &mut Parser<'a>) -> ExpressionResponse;
pub type InfixParseFn<'a> = fn(parser: &mut Parser<'a>, dyn Expression) -> ExpressionResponse;
