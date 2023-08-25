use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
    types::ASTExpression,
};

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub lhs: ASTExpression,
    pub operator: String,
    pub rhs: ASTExpression,
}

impl InfixExpression {
    pub fn new(token: Token, lhs: ASTExpression, operator: String, rhs: ASTExpression) -> Self {
        Self {
            token,
            lhs,
            operator,
            rhs,
        }
    }
}

impl Expression for InfixExpression {
    fn eval(&self) -> String {
        "".to_string()
    }
}

impl Node for InfixExpression {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }


    fn as_node(&self) -> &dyn Node {
        self
    }
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.lhs.to_string(),
            self.operator,
            self.rhs.to_string()
        )
    }
}
