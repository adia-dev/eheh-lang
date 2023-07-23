
use crate::{token::Token, traits::{expression::Expression, node::Node}};

#[derive(Debug)]
pub struct IdentifierExpression {
    pub token: Token,
    pub value: String
}

impl Expression for IdentifierExpression {
    fn eval(&self) -> String {
        todo!()
    }
}

impl Node for IdentifierExpression {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl ToString for IdentifierExpression {
    fn to_string(&self) -> String {
        self.token.literal.to_string()
    }
}

