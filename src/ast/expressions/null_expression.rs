use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug, Clone)]
pub struct NullExpression {
    pub token: Token,
}

impl NullExpression {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl Expression for NullExpression {
    fn eval(&self) -> String {
        self.get_token_literal()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for NullExpression {
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

impl ToString for NullExpression {
    fn to_string(&self) -> String {
        "null".to_string()
    }
}
