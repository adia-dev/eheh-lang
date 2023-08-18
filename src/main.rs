#![allow(dead_code)]
#![allow(unused)]

use std::any::type_name;

use colored::Colorize;
use error::parser_error::{ParserError, ParserErrorCode::UnexpectedToken};
use rlpl::RLPL;

use crate::token::{token_type::TokenType, Token};

mod ast;
mod error;
mod lexer;
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

    let mut rlpl = RLPL::new();

    rlpl.start();
}
