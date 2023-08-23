use crate::{lexer::Lexer, parser::Parser, traits::object::Object, types::EvaluatorResult};

use super::Evaluator;

#[test]
fn test_eval_integer_literal() {
    let expected: Vec<(&'static str, i64)> = vec![
        ("5", 5),
        ("10", 10),
    ];
}

fn test_eval_helper(input: &str) -> EvaluatorResult {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse().unwrap();

    Evaluator::eval(Box::new(program))
}
