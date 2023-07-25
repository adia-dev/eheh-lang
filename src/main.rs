#![allow(dead_code)]

use repl::REPL;

mod ast;
mod lexer;
mod parser;
mod program;
mod repl;
mod token;
mod traits;
mod types;

fn main() {
    let mut repl = REPL::new();

    repl.start();
}
