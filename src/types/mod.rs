use crate::{
    parser::Parser,
    traits::{expression::Expression, statement::Statement},
};

pub type ASTStatement = Box<dyn Statement>;
pub type ASTExpression = Box<dyn Expression>;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type ASTStatementResult = Result<ASTStatement>;
pub type ASTExpressionResult = Result<ASTExpression>;

pub type PrefixParseFn<'a> = fn(parser: &mut Parser<'a>) -> ASTExpressionResult;
pub type InfixParseFn<'a> = fn(parser: &mut Parser<'a>, ASTExpression) -> ASTExpressionResult;
