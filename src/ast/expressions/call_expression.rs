use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub args: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        args: Vec<Box<dyn Expression>>,
    ) -> Self {
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
}

impl Node for CallExpression {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
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
