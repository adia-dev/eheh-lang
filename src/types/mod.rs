use crate::{
    parser::Parser,
    traits::{expression::Expression, statement::Statement},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type StatementResponse = Result<Box<dyn Statement>>;
pub type ExpressionResponse = Result<Box<dyn Expression>>;
pub type PrefixParseFn<'a> = fn(parser: &mut Parser<'a>) -> ExpressionResponse;
pub type InfixParseFn<'a> = fn(parser: &mut Parser<'a>, dyn Expression) -> ExpressionResponse;
