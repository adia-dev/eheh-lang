#![allow(dead_code)]

use repl::REPL;

mod token;
mod repl;
mod program;
mod lexer;
mod ast;
mod traits;

fn main() {
    let mut repl = REPL::new();

    repl.start();
}
