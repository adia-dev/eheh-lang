#![allow(dead_code)]

use repl::REPL;

mod token;
mod repl;
mod lexer;

fn main() {
    let mut repl = REPL::new();

    repl.start();
}
