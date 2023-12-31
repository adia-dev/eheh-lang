use crate::{
    ast::statements::block_statement::BlockStatement,
    token::Token,
    traits::{expression::Expression, node::Node},
    types::ASTExpression,
};

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: ASTExpression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: ASTExpression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl Expression for IfExpression {
    fn eval(&self) -> String {
        "".to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for IfExpression {
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

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        if let Some(alternative) = &self.alternative {
            format!(
                "if {} {} else{}",
                self.condition.to_string(),
                self.consequence.to_string(),
                alternative.to_string()
            )
        } else {
            format!(
                "if {} {}",
                self.condition.to_string(),
                self.consequence.to_string()
            )
        }
    }
}
