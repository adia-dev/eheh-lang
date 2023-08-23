use crate::{
    log::error::ParserError,
    parser::Parser,
    traits::{expression::Expression, object::Object, statement::Statement},
};

pub type ASTStatement = Box<dyn Statement>;
pub type ASTExpression = Box<dyn Expression>;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type ParserResult<T> = std::result::Result<T, ParserError>;
pub type EvaluatorResult = std::result::Result<Box<dyn Object>, Box<dyn std::error::Error>>;

pub type ASTStatementResult = ParserResult<ASTStatement>;
pub type ASTExpressionResult = ParserResult<ASTExpression>;

pub type PrefixParseFn<'a> = fn(parser: &mut Parser<'a>) -> ASTExpressionResult;
pub type InfixParseFn<'a> = fn(parser: &mut Parser<'a>, ASTExpression) -> ASTExpressionResult;
