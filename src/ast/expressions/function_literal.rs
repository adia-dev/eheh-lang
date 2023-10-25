use crate::{
    ast::statements::block_statement::BlockStatement,
    token::Token,
    traits::{expression::Expression, node::Node},
};

use super::{identifier::Identifier, typed_identifier::TypedIdentifier};

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub name: Option<Identifier>,
    pub visibility: Option<Token>,
    pub parameters: Vec<TypedIdentifier>,
    pub return_type: Option<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(
        token: Token,
        name: Option<Identifier>,
        visibility: Option<Token>,
        parameters: Vec<TypedIdentifier>,
        return_type: Option<Identifier>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            name,
            visibility,
            parameters,
            return_type,
            body,
        }
    }
}

impl Expression for FunctionLiteral {
    fn eval(&self) -> String {
        "".to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for FunctionLiteral {
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

impl ToString for FunctionLiteral {
    fn to_string(&self) -> String {
        let mut str = String::new();

        if let Some(visibility) = &self.visibility {
            str.push_str(format!("{}", visibility.literal).as_str());
        }

        str.push_str("fn ");

        if let Some(name) = &self.name {
            str.push_str(format!("{}", name.get_token_literal()).as_str());
        }

        str.push_str("(");

        self.parameters.iter().enumerate().for_each(|(i, param)| {
            str.push_str(param.to_string().as_str());
            if i < self.parameters.len() - 1 {
                str.push_str(", ");
            }
        });

        str.push_str(")");

        if let Some(return_type) = &self.return_type {
            str.push_str(format!(" -> {} ", return_type.get_token_literal()).as_str());
        }

        str.push_str(self.body.to_string().as_str());

        str
    }
}
