#![allow(dead_code)]

use rlpl::RLPL;

mod ast;
mod lexer;
mod parser;
// mod error;
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
