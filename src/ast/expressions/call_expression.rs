use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
    types::ASTExpression,
};

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: ASTExpression,
    pub args: Vec<ASTExpression>,
}

impl CallExpression {
    pub fn new(token: Token, function: ASTExpression, args: Vec<ASTExpression>) -> Self {
        Self {
            token,
            function,
            args,
        }
    }
}

impl Expression for CallExpression {
    fn eval(&self) -> String {
        "".to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for CallExpression {
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

impl ToString for CallExpression {
    fn to_string(&self) -> String {
        let mut call_str = String::new();

        call_str.push_str(self.function.to_string().as_str());
        call_str.push_str(
            format!(
                "({})",
                self.args
                    .iter()
                    .map(|arg| { arg.to_string() })
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );

        call_str
    }
}
