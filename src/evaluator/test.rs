use crate::{
    lexer::Lexer,
    objects::{boolean::Boolean, integer::Integer, null::Null, error::Error},
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
        test_eval_integer_helper(object, value, Some(t));
    }
}

#[test]
fn test_eval_boolean() {
    let expected: Vec<(&'static str, bool)> = vec![("true", true), ("false", false)];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_boolean_helper(object, input, value);
    }
}

#[test]
fn test_eval_bang_expression() {
    let expected: Vec<(&str, &str, bool)> = vec![
        ("!true", "false", false),
        ("!false", "true", true),
        ("!!false", "false", false),
        ("!!true", "true", true),
        ("!5", "false", false),
        ("!0", "true", true),
        ("!!5", "true", true),
        ("!null", "true", true),
        ("!-(++99)", "false", false),
    ];

    for (input, value_str, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_boolean_helper(object, value_str, value);
    }
}

#[test]
fn test_eval_boolean_expression() {
    let expected: Vec<(&str, bool)> = vec![
        ("true && true", true),
        ("true && false", false),
        ("true || false", true),
        ("false || false", false),
        ("true && 1", true),
        ("true && 0", false),
        ("1 || 0", true),
        ("1 && 5", true),
        ("(1 && 5 && false) || true", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_boolean_helper(object, value.to_string().as_str(), value);
    }
}

#[test]
fn test_eval_return_statement() {
    let expected: Vec<(&str, i64)> = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        ("if true { if true { return 1; } return 2; }", 1),
    ];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_integer_helper(object, value, None);
    }
}

#[test]
fn test_eval_if_expression() {
    let expected: Vec<(&str, Box<dyn Object>)> = vec![
        ("if (true) { 10 }", Box::new(Integer::new(10))),
        ("if (false) { 10 }", Box::new(Null::new())),
        ("if (1) { 10 }", Box::new(Integer::new(10))),
        ("if (1 < 2) { 10 }", Box::new(Integer::new(10))),
        ("if (1 > 2) { 10 }", Box::new(Null::new())),
        ("if (1 > 2) { 10 } else { 20 }", Box::new(Integer::new(20))),
        ("if (1 < 2) { 10 } else { 20 }", Box::new(Integer::new(10))),
    ];

    for (input, expected_object) in expected {
        let object = test_eval_helper(input).unwrap();
        if object.t() == ObjectType::Null {
            test_downcast_object_helper::<Null>(&expected_object);
        } else {
            let integer = test_downcast_object_helper::<Integer>(&expected_object);
            let expected_integer = test_downcast_object_helper::<Integer>(&expected_object);
            test_eval_integer_helper(object, expected_integer.value, Some(expected_integer.t()));
        }
    }
}

#[test]
fn test_eval_integer_expression() {
    let expected: Vec<(&str, i64)> = vec![
        ("5", 5),
        ("-5", -5),
        ("-(-5)", 5),
        ("--5", 4),
        ("++5", 6),
        ("-(++99)", -100),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_eval_integer_helper(object, value, None);
    }
}

#[test]
fn test_error_handling() {
    let expected: Vec<(&str, &str)> = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
    ];

    for (input, value) in expected {
        let object = test_eval_helper(input).unwrap();
        test_downcast_object_helper::<Error>(&object);
        println!("{:?}", object);
    }
}

fn test_eval_helper(input: &str) -> EvaluatorResult {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse().unwrap();

    Evaluator::eval(Box::new(program.as_node()))
}

fn test_eval_integer_helper(object: Box<dyn Object>, value: i64, t: Option<ObjectType>) -> Integer {
    let integer = test_downcast_object_helper::<Integer>(&object);

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

fn test_eval_boolean_helper(object: Box<dyn Object>, input: &str, value: bool) -> Boolean {
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
    match object.as_any_ref().downcast_ref::<T>() {
        Some(t_exp) => t_exp,
        None => {
            panic!("Failed to downcast an object: {:?}", object.to_string())
        }
    }
}
