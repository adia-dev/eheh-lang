use std::any::Any;

use crate::{
    token::Token,
    traits::{expression::Expression, node::Node, statement::Statement},
};

#[derive(Debug)]
pub struct ExpressionStatetmentStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatetmentStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl Node for ExpressionStatetmentStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatetmentStatement {
    fn process(&self) {}
}

impl ToString for ExpressionStatetmentStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}
