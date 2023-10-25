use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
    types::ASTExpression,
};

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub rhs: ASTExpression,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, rhs: ASTExpression) -> Self {
        Self {
            token,
            operator,
            rhs,
        }
    }
}

impl Expression for PrefixExpression {
    fn eval(&self) -> String {
        "".to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for PrefixExpression {
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

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.rhs.to_string())
    }
}
