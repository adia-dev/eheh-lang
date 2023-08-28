use core::panic;
use std::sync::Arc;

use crate::{
    ast::{
        expressions::{
            boolean_expression::BooleanExpression, identifier::Identifier,
            if_expression::IfExpression, infix_expression::InfixExpression,
            integer_literal::IntegerLiteral, prefix_expression::PrefixExpression,
        },
        statements::{
            block_statement::BlockStatement, declare_statement::DeclareStatement,
            expression_statement::ExpressionStatement, return_statement::ReturnStatement,
        },
    },
    log::error::runtime::{RuntimeError, RuntimeErrorCode},
    objects::{
        boolean::Boolean,
        environment::{self, Environment},
        error::Error,
        integer::Integer,
        null::Null,
        return_::Return,
    },
    program::Program,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
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

static ZERO: Integer = Integer::new(0);
static ONE: Integer = Integer::new(1);

static NULL: Null = Null::new();

impl Evaluator {
    pub fn eval<'a>(node: Box<&dyn Node>, environment: &'a mut Environment) -> EvaluatorResult {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            return Evaluator::eval_program(&program.statements, environment);
        }

        if let Some(block) = node.as_any().downcast_ref::<BlockStatement>() {
            return Evaluator::eval_block_statement(&block.statements, environment);
        }

        if let Some(exp_stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
            return Evaluator::eval(Box::new(exp_stmt.expression.as_node()), environment);
        }

        if let Some(declare_stmt) = node.as_any().downcast_ref::<DeclareStatement>() {
            if let Some(exp) = &declare_stmt.value {
                let declare_value = Evaluator::eval(Box::new(exp.as_node()), environment)?;

                if Evaluator::is_error(&declare_value) {
                    return Ok(declare_value);
                }
                environment.set(declare_stmt.name.value.clone(), &declare_value);

                return Ok(Box::new(Return::new(Some(declare_value))));
            } else {
                return Ok(Box::new(NULL.clone()));
            }
        }

        if let Some(return_stmt) = node.as_any().downcast_ref::<ReturnStatement>() {
            if let Some(exp) = &return_stmt.value {
                let return_value = Evaluator::eval(Box::new(exp.as_node()), environment)?;

                if Evaluator::is_error(&return_value) {
                    return Ok(return_value);
                }

                return Ok(Box::new(Return::new(Some(return_value))));
            } else {
                return Ok(Box::new(NULL.clone()));
            }
        }

        if let Some(identifier) = node.as_any().downcast_ref::<Identifier>() {
            return Evaluator::eval_identifier(&identifier, environment);
        }

        if let Some(if_exp) = node.as_any().downcast_ref::<IfExpression>() {
            return Evaluator::eval_if_expression(&if_exp, environment);
        }

        if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            return Ok(Box::new(Integer::new(integer_literal.value)));
        }

        if let Some(boolean) = node.as_any().downcast_ref::<BooleanExpression>() {
            if boolean.value {
                return Ok(Box::new(TRUE.clone()));
            } else {
                return Ok(Box::new(FALSE.clone()));
            }
        }

        if let Some(infix_expression) = node.as_any().downcast_ref::<InfixExpression>() {
            let lhs = Evaluator::eval(Box::new(infix_expression.lhs.as_node()), environment)?;

            if Evaluator::is_error(&lhs) {
                return Ok(lhs);
            }

            let rhs = Evaluator::eval(Box::new(infix_expression.rhs.as_node()), environment)?;
            if Evaluator::is_error(&rhs) {
                return Ok(rhs);
            }
            return Evaluator::eval_infix_expression(infix_expression.operator.as_str(), lhs, rhs);
        }

        if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            let rhs = Evaluator::eval(Box::new(prefix_expression.rhs.as_node()), environment)?;
            if Evaluator::is_error(&rhs) {
                return Ok(rhs);
            }

            return Evaluator::eval_prefix_expression(prefix_expression.operator.as_str(), rhs);
        }

        return Ok(Box::new(NULL.clone()));
    }

    fn eval_program(
        statements: &Vec<ASTStatement>,
        environment: &mut Environment,
    ) -> EvaluatorResult {
        if statements.is_empty() {
            return Ok(Box::new(NULL.clone()));
        }

        let mut object: Option<Box<dyn Object>> = None;

        for (stmt) in statements {
            let evaluated = Evaluator::eval(Box::new(stmt.as_node()), environment)?;

            match evaluated.t() {
                ObjectType::Return => {
                    if let Some(return_value) =
                        &Evaluator::downcast_ref_object::<Return>(&evaluated).value
                    {
                        return Ok(return_value.clone());
                    } else {
                        return Ok(Box::new(NULL.clone()));
                    }
                }
                ObjectType::Error => {
                    return Ok(evaluated);
                }
                _ => (),
            }

            object = Some(evaluated);
        }

        if let Some(obj) = object {
            Ok(obj)
        } else {
            Ok(Box::new(NULL.clone()))
        }
    }

    fn eval_block_statement(
        statements: &Vec<ASTStatement>,
        environment: &mut Environment,
    ) -> EvaluatorResult {
        if statements.is_empty() {
            return Ok(Box::new(NULL.clone()));
        }

        let mut object: Option<Box<dyn Object>> = None;

        for (stmt) in statements {
            let evaluated = Evaluator::eval(Box::new(stmt.as_node()), environment)?;

            if evaluated.t() == ObjectType::Return || evaluated.t() == ObjectType::Error {
                return Ok(evaluated);
            }

            object = Some(evaluated);
        }

        if let Some(obj) = object {
            Ok(obj)
        } else {
            Ok(Box::new(NULL.clone()))
        }
    }

    fn eval_identifier(identifier: &Identifier, environment: &mut Environment) -> EvaluatorResult {
        if let Some(value) = environment.get(identifier.value.clone()) {
            Ok(value.clone())
        } else {
            Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::IdentifierNotFound {
                    context: Some(identifier.value.clone()),
                    identifier: identifier.value.clone(),
                },
                source: None,
            }))
        }
    }

    fn eval_if_expression(if_exp: &IfExpression, environment: &mut Environment) -> EvaluatorResult {
        let condition = Evaluator::eval(Box::new(if_exp.condition.as_node()), environment)?;

        if Evaluator::is_truthy(&condition) {
            return Evaluator::eval(Box::new(if_exp.consequence.as_node()), environment);
        } else if let Some(alt) = &if_exp.alternative {
            return Evaluator::eval(Box::new(alt.as_node()), environment);
        } else {
            return Ok(Box::new(NULL.clone()));
        }
    }

    fn eval_prefix_expression(operator: &str, mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match operator {
            "!" => Evaluator::eval_bang_prefix_expression(rhs),
            "-" => Evaluator::eval_minus_prefix_expression(rhs),
            "--" => Evaluator::eval_decr_prefix_expression(rhs),
            "++" => Evaluator::eval_incr_prefix_expression(rhs),
            ".." | "..=" => match rhs.t() {
                ObjectType::Integer(_) => Evaluator::eval_integer_to_integer_infix_expression(
                    operator,
                    Box::new(Integer::new(0)),
                    rhs,
                ),
                _ => Evaluator::new_error(Box::new(RuntimeError {
                    code: RuntimeErrorCode::InvalidOperation {
                        operation: format!("{}{}", operator, rhs.to_string()),
                        context: None,
                    },
                    source: None,
                })),
            },
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::UnknownInfixOperator {
                    operator: operator.to_string(),
                    context: Some(format!(
                        "{}{}({})",
                        operator,
                        rhs.to_string(),
                        rhs.t().to_string(),
                    )),
                },
                source: None,
            })),
        }
    }

    fn eval_bang_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Boolean => {
                let mut boolean = Evaluator::downcast_mut_object::<Boolean>(&mut rhs);
                boolean.value = !boolean.value;
                Ok(rhs)
            }
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_ref_object::<Integer>(&rhs);
                if integer.value == 0 {
                    Ok(Box::new(TRUE.clone()))
                } else {
                    Ok(Box::new(FALSE.clone()))
                }
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::InvalidOperation {
                    operation: format!("!{}", rhs.to_string()),
                    context: Some(format!("!{}", rhs.to_string())),
                },
                source: None,
            })),
        }
    }

    fn eval_minus_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_mut_object::<Integer>(&mut rhs);
                integer.value = -integer.value;
                Ok(rhs)
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::InvalidOperation {
                    operation: format!("-{}", rhs.to_string()),
                    context: Some(format!("-{}", rhs.to_string())),
                },
                source: None,
            })),
        }
    }

    fn eval_incr_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_mut_object::<Integer>(&mut rhs);
                let (value, overflowed) = integer.value.overflowing_add(1);
                if overflowed {
                    Evaluator::new_error(Box::new(RuntimeError {
                        code: RuntimeErrorCode::OverflowError,
                        source: None,
                    }))
                } else {
                    integer.value = value;
                    Ok(rhs)
                }
            }
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::InvalidOperation {
                    operation: format!("++{}", rhs.to_string()),
                    context: Some(format!("++{}", rhs.to_string())),
                },
                source: None,
            })),
        }
    }

    fn eval_decr_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_mut_object::<Integer>(&mut rhs);
                let (value, overflowed) = integer.value.overflowing_sub(1);
                if overflowed {
                    Evaluator::new_error(Box::new(RuntimeError {
                        code: RuntimeErrorCode::OverflowError,
                        source: None,
                    }))
                } else {
                    integer.value = value;
                    Ok(rhs)
                }
            }
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::InvalidOperation {
                    operation: format!("--{}", rhs.to_string()),
                    context: Some(format!("--{}", rhs.to_string())),
                },
                source: None,
            })),
        }
    }

    fn eval_infix_expression(
        operator: &str,
        mut lhs: Box<dyn Object>,
        mut rhs: Box<dyn Object>,
    ) -> EvaluatorResult {
        match (lhs.t(), rhs.t()) {
            (ObjectType::Boolean, ObjectType::Boolean) => {
                Evaluator::eval_boolean_to_boolean_infix_expression(operator, lhs, rhs)
            }
            (ObjectType::Integer(_), ObjectType::Integer(_)) => {
                Evaluator::eval_integer_to_integer_infix_expression(operator, lhs, rhs)
            }
            (ObjectType::Boolean, ObjectType::Integer(_)) => {
                let rhs_integer = Evaluator::downcast_ref_object::<Integer>(&rhs);
                match operator {
                    "&&" | "||" | "==" | "!=" | ">" | "<" | ">=" | "<=" => {
                        Evaluator::eval_boolean_to_boolean_infix_expression(
                            operator,
                            lhs,
                            Box::new(Boolean::new(rhs_integer.value != 0)),
                        )
                    }
                    _ => {
                        let lhs_boolean = Evaluator::downcast_ref_object::<Boolean>(&lhs);
                        let lhs_integer = Integer::new(if lhs_boolean.value { 1 } else { 0 });

                        Evaluator::eval_integer_to_integer_infix_expression(
                            operator,
                            Box::new(lhs_integer),
                            rhs,
                        )
                    }
                }
            }

            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::InvalidOperation {
                    context: None,
                    operation: format!(
                        "{}({}){}{}({})",
                        lhs.to_string(),
                        lhs.t().to_string(),
                        operator,
                        rhs.to_string(),
                        rhs.t().to_string()
                    ),
                },
                source: None,
            })),
        }
    }

    fn eval_boolean_to_boolean_infix_expression(
        operator: &str,
        mut lhs: Box<dyn Object>,
        mut rhs: Box<dyn Object>,
    ) -> EvaluatorResult {
        let lhs_boolean = Evaluator::downcast_ref_object::<Boolean>(&lhs);
        let rhs_boolean = Evaluator::downcast_ref_object::<Boolean>(&rhs);

        match operator {
            "&&" => Ok(Box::new(Boolean::new(
                lhs_boolean.value && rhs_boolean.value,
            ))),
            "||" => Ok(Box::new(Boolean::new(
                lhs_boolean.value || rhs_boolean.value,
            ))),
            "==" => Ok(Box::new(Boolean::new(
                lhs_boolean.value == rhs_boolean.value,
            ))),
            "!=" => Ok(Box::new(Boolean::new(
                lhs_boolean.value != rhs_boolean.value,
            ))),
            "<" => Ok(Box::new(Boolean::new(
                lhs_boolean.value < rhs_boolean.value,
            ))),
            ">" => Ok(Box::new(Boolean::new(
                lhs_boolean.value > rhs_boolean.value,
            ))),
            "<=" => Ok(Box::new(Boolean::new(
                lhs_boolean.value <= rhs_boolean.value,
            ))),
            ">=" => Ok(Box::new(Boolean::new(
                lhs_boolean.value >= rhs_boolean.value,
            ))),
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::UnknownInfixOperator {
                    operator: operator.to_string(),
                    context: Some(format!(
                        "{}({}){}{}({})",
                        lhs.to_string(),
                        lhs.t().to_string(),
                        operator,
                        rhs.to_string(),
                        rhs.t().to_string()
                    )),
                },
                source: None,
            })),
        }
    }

    fn eval_integer_to_integer_infix_expression(
        operator: &str,
        mut lhs: Box<dyn Object>,
        mut rhs: Box<dyn Object>,
    ) -> EvaluatorResult {
        let lhs_integer = Evaluator::downcast_ref_object::<Integer>(&lhs);
        let rhs_integer = Evaluator::downcast_ref_object::<Integer>(&rhs);

        match operator {
            "&&" | "||" => {
                let lhs_boolean = Boolean::new(lhs_integer.value != 0);
                let rhs_boolean = Boolean::new(rhs_integer.value != 0);

                Evaluator::eval_boolean_to_boolean_infix_expression(
                    operator,
                    Box::new(lhs_boolean),
                    Box::new(rhs_boolean),
                )
            }
            "==" => Ok(Box::new(Boolean::new(
                lhs_integer.value == rhs_integer.value,
            ))),
            "!=" => Ok(Box::new(Boolean::new(
                lhs_integer.value != rhs_integer.value,
            ))),
            ">" => Ok(Box::new(Boolean::new(
                lhs_integer.value > rhs_integer.value,
            ))),
            "<" => Ok(Box::new(Boolean::new(
                lhs_integer.value < rhs_integer.value,
            ))),
            ">=" => Ok(Box::new(Boolean::new(
                lhs_integer.value >= rhs_integer.value,
            ))),
            "<=" => Ok(Box::new(Boolean::new(
                lhs_integer.value <= rhs_integer.value,
            ))),
            ".." => Ok(Box::new(Integer::new(
                rhs_integer.value - lhs_integer.value,
            ))),
            "..=" => Ok(Box::new(Integer::new(
                rhs_integer.value - lhs_integer.value + 1,
            ))),
            "+" => Ok(Box::new(Integer::new(
                lhs_integer.value + rhs_integer.value,
            ))),
            "-" => Ok(Box::new(Integer::new(
                lhs_integer.value - rhs_integer.value,
            ))),
            "*" => Ok(Box::new(Integer::new(
                lhs_integer.value * rhs_integer.value,
            ))),
            "%" => Ok(Box::new(Integer::new(
                lhs_integer.value % rhs_integer.value,
            ))),
            "^" | "**" => Ok(Box::new(Integer::new(
                lhs_integer.value.pow(rhs_integer.value as u32),
            ))),
            "/" => Ok(Box::new(Integer::new(
                lhs_integer.value / rhs_integer.value,
            ))),
            _ => Evaluator::new_error(Box::new(RuntimeError {
                code: RuntimeErrorCode::UnknownInfixOperator {
                    operator: operator.to_string(),
                    context: Some(format!(
                        "{}({}){}{}({})",
                        lhs.to_string(),
                        lhs.t().to_string(),
                        operator,
                        rhs.to_string(),
                        rhs.t().to_string(),
                    )),
                },
                source: None,
            })),
        }
    }

    pub fn downcast_ref_object<T: 'static>(object: &Box<dyn Object>) -> &T {
        match object.as_any_ref().downcast_ref::<T>() {
            Some(obj) => obj,
            None => {
                panic!("Failed to downcast an object: {:?}", object.to_string())
            }
        }
    }

    pub fn downcast_mut_object<T: 'static>(object: &mut Box<dyn Object>) -> &mut T {
        match object.as_any_mut().downcast_mut::<T>() {
            Some(obj) => obj,
            None => {
                panic!("Failed to downcast an object to a mutable reference.")
            }
        }
    }

    pub fn is_truthy(object: &Box<dyn Object>) -> bool {
        match object.t() {
            ObjectType::Boolean => Evaluator::downcast_ref_object::<Boolean>(&object).value,
            ObjectType::Integer(_) => Evaluator::downcast_ref_object::<Integer>(&object).value != 0,
            _ => false,
        }
    }

    pub fn is_error(object: &Box<dyn Object>) -> bool {
        object.t() == ObjectType::Error
    }

    pub fn new_error(error: Box<dyn std::error::Error>) -> EvaluatorResult {
        let i_use_arch_btw_wrapped_error: Arc<dyn std::error::Error> =
            Arc::<dyn std::error::Error>::from(error);
        Ok(Box::new(Error::new(i_use_arch_btw_wrapped_error)))
    }
}

#[cfg(test)]
pub mod test;
