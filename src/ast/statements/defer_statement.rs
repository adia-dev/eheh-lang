use std::any::Any;

use crate::{
    token::Token,
    traits::{node::Node, statement::Statement},
    types::ASTExpression,
};

#[derive(Debug, Clone)]
pub struct DeferStatement {
    pub token: Token,
    pub value: ASTExpression,
}

impl DeferStatement {
    pub fn new(token: Token, value: ASTExpression) -> Self {
        Self { token, value }
    }
}

impl Node for DeferStatement {
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

impl Statement for DeferStatement {
    fn process(&self) {}
    fn clone_boxed(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl ToString for DeferStatement {
    fn to_string(&self) -> String {
        format!("Defer {};", self.value.to_string())
    }
}
