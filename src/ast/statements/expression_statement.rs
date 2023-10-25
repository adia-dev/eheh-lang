use std::any::Any;

use crate::{
    token::Token,
    traits::{node::Node, statement::Statement},
    types::ASTExpression,
};

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ASTExpression,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: ASTExpression) -> Self {
        Self { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
}

impl Statement for ExpressionStatement {
    fn process(&self) {}
    fn clone_boxed(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}
