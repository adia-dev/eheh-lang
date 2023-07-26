use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub rhs: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, rhs: Box<dyn Expression>) -> Self {
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
}

impl Node for PrefixExpression {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.rhs.to_string())
    }
}
