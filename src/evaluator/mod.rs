use core::panic;

use crate::{
    ast::{
        expressions::{
            boolean_expression::BooleanExpression, integer_literal::IntegerLiteral,
            prefix_expression::PrefixExpression,
        },
        statements::expression_statements::ExpressionStatement,
    },
    objects::{boolean::Boolean, integer::Integer, null::Null},
    program::Program,
    traits::{
        node::Node,
        object::{Object, ObjectType},
    },
    types::{ASTStatement, EvaluatorResult},
};

#[derive(Debug, Clone)]
pub struct Evaluator {}

static TRUE: Boolean = Boolean::new(true);
static FALSE: Boolean = Boolean::new(false);
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

        if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            let rhs = Evaluator::eval(Box::new(prefix_expression.rhs.as_node()))?;
            return Evaluator::eval_prefix_expression(prefix_expression.operator.as_str(), rhs);
        }

        if let Some(boolean) = node.as_any().downcast_ref::<BooleanExpression>() {
            if boolean.value {
                return Ok(Box::new(TRUE.clone()));
            } else {
                return Ok(Box::new(FALSE.clone()));
            }
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

    fn eval_prefix_expression(operator: &str, mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match operator {
            "!" => Evaluator::eval_bang_prefix_expression(rhs),
            "-" => Evaluator::eval_minus_prefix_expression(rhs),
            "--" => Evaluator::eval_decr_prefix_expression(rhs),
            "++" => Evaluator::eval_incr_prefix_expression(rhs),
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    fn eval_bang_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Boolean => {
                let mut boolean = Evaluator::downcast_object_mut::<Boolean>(&mut rhs);
                boolean.value = !boolean.value;
                Ok(rhs)
            }
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_object::<Integer>(&rhs);
                if integer.value == 0 {
                    Ok(Box::new(TRUE.clone()))
                } else {
                    Ok(Box::new(FALSE.clone()))
                }
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    fn eval_minus_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_object_mut::<Integer>(&mut rhs);
                integer.value = -integer.value;
                Ok(rhs)
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    fn eval_incr_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_object_mut::<Integer>(&mut rhs);
                integer.value = integer.value.overflowing_add(1).0;
                Ok(rhs)
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    fn eval_decr_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_object_mut::<Integer>(&mut rhs);
                integer.value = integer.value.overflowing_sub(1).0;
                Ok(rhs)
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    pub fn downcast_object<T: 'static>(object: &Box<dyn Object>) -> &T {
        match object.as_any().downcast_ref::<T>() {
            Some(t_exp) => t_exp,
            None => {
                panic!("Failed to downcast an object: {:?}", object.to_string())
            }
        }
    }

    pub fn downcast_object_mut<T: 'static>(object: &mut Box<dyn Object>) -> &mut T {
        match object.as_any_mut().downcast_mut::<T>() {
            Some(t_exp) => t_exp,
            None => {
                panic!("Failed to downcast an object to a mutable reference.")
            }
        }
    }
}

#[cfg(test)]
pub mod test;
