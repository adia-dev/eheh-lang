use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        Self { token, value }
    }

    pub fn from_token(token: &Token) -> Self {
        let value = token.literal.parse::<i64>().unwrap();

        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for IntegerLiteral {
    fn eval(&self) -> String {
        self.value.to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for IntegerLiteral {
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

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
