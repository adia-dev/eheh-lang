use core::panic;

use crate::{
    ast::{
        expressions::{
            boolean_expression::BooleanExpression, if_expression::IfExpression,
            infix_expression::InfixExpression, integer_literal::IntegerLiteral,
            prefix_expression::PrefixExpression,
        },
        statements::{
            block_statement::BlockStatement, expression_statement::ExpressionStatement,
            return_statement::ReturnStatement,
        },
    },
    objects::{boolean::Boolean, integer::Integer, null::Null, return_::Return},
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

static ZERO: Integer = Integer::new(0);
static ONE: Integer = Integer::new(1);

static NULL: Null = Null::new();

impl Evaluator {
    pub fn eval(node: Box<&dyn Node>) -> EvaluatorResult {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            return Evaluator::eval_program(&program.statements);
        }

        if let Some(block) = node.as_any().downcast_ref::<BlockStatement>() {
            return Evaluator::eval_block_statement(&block.statements);
        }

        if let Some(exp_stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
            return Evaluator::eval(Box::new(exp_stmt.expression.as_node()));
        }

        if let Some(return_stmt) = node.as_any().downcast_ref::<ReturnStatement>() {
            if let Some(exp) = &return_stmt.value {
                let return_value = Evaluator::eval(Box::new(exp.as_node()))?;
                return Ok(Box::new(Return::new(Some(return_value))));
            } else {
                return Ok(Box::new(NULL.clone()));
            }
        }

        if let Some(if_exp) = node.as_any().downcast_ref::<IfExpression>() {
            return Evaluator::eval_if_expression(&if_exp);
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
            let lhs = Evaluator::eval(Box::new(infix_expression.lhs.as_node()))?;
            let rhs = Evaluator::eval(Box::new(infix_expression.rhs.as_node()))?;
            return Evaluator::eval_infix_expression(infix_expression.operator.as_str(), lhs, rhs);
        }

        if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            let rhs = Evaluator::eval(Box::new(prefix_expression.rhs.as_node()))?;
            return Evaluator::eval_prefix_expression(prefix_expression.operator.as_str(), rhs);
        }

        return Ok(Box::new(NULL.clone()));
    }

    fn eval_program(statements: &Vec<ASTStatement>) -> EvaluatorResult {
        if statements.is_empty() {
            return Ok(Box::new(NULL.clone()));
        }

        let mut object: Option<Box<dyn Object>> = None;

        for (stmt) in statements {
            let evaluated = Evaluator::eval(Box::new(stmt.as_node()))?;

            if evaluated.t() == ObjectType::Return {
                if let Some(return_value) =
                    &Evaluator::downcast_ref_object::<Return>(&evaluated).value
                {
                    return Ok(return_value.clone());
                } else {
                    return Ok(Box::new(NULL.clone()));
                }
            }

            object = Some(evaluated);
        }

        if let Some(obj) = object {
            Ok(obj)
        } else {
            Ok(Box::new(NULL.clone()))
        }
    }

    fn eval_block_statement(statements: &Vec<ASTStatement>) -> EvaluatorResult {
        if statements.is_empty() {
            return Ok(Box::new(NULL.clone()));
        }

        let mut object: Option<Box<dyn Object>> = None;

        for (stmt) in statements {
            let evaluated = Evaluator::eval(Box::new(stmt.as_node()))?;

            if evaluated.t() == ObjectType::Return {
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

    fn eval_if_expression(if_exp: &IfExpression) -> EvaluatorResult {
        let condition = Evaluator::eval(Box::new(if_exp.condition.as_node()))?;

        if Evaluator::is_truthy(&condition) {
            return Evaluator::eval(Box::new(if_exp.consequence.as_node()));
        } else if let Some(alt) = &if_exp.alternative {
            return Evaluator::eval(Box::new(alt.as_node()));
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
            ".." | "..=" => Evaluator::eval_integer_to_integer_infix_expression(
                operator,
                Box::new(Integer::new(0)),
                rhs,
            ),
            _ => Ok(Box::new(NULL.clone())),
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
            _ => Ok(Box::new(NULL.clone())),
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
            _ => Ok(Box::new(NULL.clone())),
        }
    }

    fn eval_incr_prefix_expression(mut rhs: Box<dyn Object>) -> EvaluatorResult {
        match rhs.t() {
            ObjectType::Integer(_) => {
                let mut integer = Evaluator::downcast_mut_object::<Integer>(&mut rhs);
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
                let mut integer = Evaluator::downcast_mut_object::<Integer>(&mut rhs);
                integer.value = integer.value.overflowing_sub(1).0;
                Ok(rhs)
            }
            ObjectType::Null => Ok(Box::new(TRUE.clone())),
            _ => Ok(Box::new(NULL.clone())),
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
            _ => Ok(Box::new(NULL.clone())),
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
            _ => Ok(Box::new(NULL.clone())),
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
            _ => Ok(Box::new(NULL.clone())),
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
}

#[cfg(test)]
pub mod test;
