use core::panic;

use crate::{
    ast::{
        expressions::{boolean_expression::BooleanExpression, integer_literal::IntegerLiteral},
        statements::expression_statements::ExpressionStatement,
    },
    objects::{boolean::Boolean, integer::Integer, null::Null},
    program::Program,
    traits::{node::Node, object::Object},
    types::{ASTStatement, EvaluatorResult},
};

#[derive(Debug, Clone)]
pub struct Evaluator {}

// can't take full advantage of it since I have to clone them everytime...
static TRUE: Boolean = Boolean::new(true);
static FALSE: Boolean = Boolean::new(true);
static NULL: Null = Null::new();

impl Evaluator {
    pub fn eval(node: Box<&dyn Node>) -> EvaluatorResult {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            return Evaluator::eval_statements(&program.statements);
        }

        if let Some(exp_stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
            return Evaluator::eval(Box::new(exp_stmt.expression.as_node()));
        }

        if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            return Ok(Box::new(Integer::new(integer_literal.value)));
        }

        if let Some(boolean) = node.as_any().downcast_ref::<BooleanExpression>() {
            return Ok(Box::new(TRUE.clone()));
        }

        return Ok(Box::new(NULL.clone()));
    }

    fn eval_statements(statements: &Vec<ASTStatement>) -> EvaluatorResult {
        if statements.is_empty() {
            return Ok(Box::new(NULL.clone()));
        }

        let mut object: Option<EvaluatorResult> = None;

        for (stmt) in statements {
            object = Some(Evaluator::eval(Box::new(stmt.as_node())));
        }

        return object.unwrap();
    }
}

#[cfg(test)]
pub mod test;
