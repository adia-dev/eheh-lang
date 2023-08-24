use std::process::exit;

use crate::{
    lexer::Lexer,
    objects::{boolean::Boolean, integer::Integer},
    parser::Parser,
    traits::{
        node::Node,
        object::{IntegerType, Object, ObjectType},
    },
    types::EvaluatorResult,
};

use super::Evaluator;

#[test]
fn test_eval_integer_literal() {
    let expected: Vec<(&'static str, i64, ObjectType)> = vec![
        ("5", 5, ObjectType::Integer(IntegerType::I64)),
        ("10", 10, ObjectType::Integer(IntegerType::I64)),
    ];

    for (input, value, t) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_integer_helper(input, value, Some(t), object);
    }
}

#[test]
fn test_eval_boolean() {
    let expected: Vec<(&'static str, bool)> = vec![
        ("true", true),
        ("false", false),
    ];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_boolean_helper(input, value, object);
    }
}

fn test_eval_helper(input: &str) -> EvaluatorResult {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse().unwrap();

    Evaluator::eval(Box::new(program.as_node()))
}

fn test_eval_integer_helper(
    input: &str,
    value: i64,
    t: Option<ObjectType>,
    object: Box<dyn Object>,
) -> Integer {
    let integer = test_downcast_object_helper::<Integer>(&object);

    assert_eq!(
        integer.to_string(),
        input,
        "Expected integer representation: {}, but got: {}",
        input,
        integer.to_string()
    );
    assert_eq!(
        integer.value, value,
        "Expected integer value: {}, but got: {}",
        value, integer.value
    );
    if let Some(expected_type) = t {
        assert_eq!(
            integer.t(),
            expected_type,
            "Expected integer type: {}, but got: {}",
            expected_type,
            integer.t()
        );
    }

    integer.clone()
}

fn test_eval_boolean_helper(input: &str, value: bool, object: Box<dyn Object>) -> Boolean {
    let boolean = test_downcast_object_helper::<Boolean>(&object);

    assert_eq!(
        boolean.to_string(),
        input,
        "Expected boolean representation: {}, but got: {}",
        input,
        boolean.to_string()
    );
    assert_eq!(
        boolean.value, value,
        "Expected Boolean value: {}, but got: {}",
        value, boolean.value
    );

    boolean.clone()
}

fn test_downcast_object_helper<T: 'static>(object: &Box<dyn Object>) -> &T {
    match object.as_any().downcast_ref::<T>() {
        Some(t_exp) => t_exp,
        None => {
            panic!("Failed to downcast an object: {:?}", object.to_string())
        }
    }
}
