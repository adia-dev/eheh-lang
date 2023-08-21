#![allow(unused)]

use rlpl::RLPL;

use crate::{
    log::warning::ParserWarning,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
};

mod ast;
mod lexer;
mod log;
mod parser;
mod program;
mod repl;
mod rlpl;
mod token;
mod traits;
mod types;

fn main() {
    // let mut repl = REPL::new();

    // repl.start();

    // let mut rlpl = RLPL::new();
    // rlpl.start();

    let token = Token::new(
        TokenType::IDENT,
        "a + b".to_string(),
        2,
        16,
        Some("src/main.rs".to_string()),
    );

    let around = Token::new(
        TokenType::KEYWORD(KeywordTokenType::IF),
        "if".to_string(),
        10,
        10,
        Some("src/main.rs".to_string()),
    );

    let context: Vec<String> = r#"
            if (a + b) {
                false
            } else {
                true
            }
        "#
    .to_string()
    .split("\n")
    .map(|s| s.to_string())
    .collect();

    let warning = ParserWarning::UnnecessaryParentheses {
        token,
        around,
        context,
    };

    println!("{}", warning);
}
