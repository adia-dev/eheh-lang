use std::any::Any;

use crate::{
    token::Token,
    traits::{node::Node, statement::Statement},
    types::ASTExpression,
};

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<ASTExpression>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Option<ASTExpression>) -> Self {
        Self { token, value }
    }
}

impl Node for ReturnStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }


    fn as_node(&self) -> &dyn Node {
        self
    }
}

impl Statement for ReturnStatement {
    fn process(&self) {}
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        if let Some(val) = &self.value {
            format!("return {};", val.to_string())
        } else {
            format!("return;")
        }
    }
}
