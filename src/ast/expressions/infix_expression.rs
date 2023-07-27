use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub lhs: Box<dyn Expression>,
    pub operator: String,
    pub rhs: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> Self {
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

    fn as_any(&self) -> &dyn std::any::Any {
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
