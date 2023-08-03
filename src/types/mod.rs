use crate::{
    parser::Parser,
    traits::{expression::Expression, statement::Statement},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type StatementResult = Result<Box<dyn Statement>>;
pub type ExpressionResult = Result<Box<dyn Expression>>;
pub type PrefixParseFn<'a> = fn(parser: &mut Parser<'a>) -> ExpressionResult;
pub type InfixParseFn<'a> = fn(parser: &mut Parser<'a>, Box<dyn Expression>) -> ExpressionResult;
