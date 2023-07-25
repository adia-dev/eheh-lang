// TODO: Intercept the signal such as Ctrl+C or others to act override the default behavior
// TODO: Rewrite the help command so that it actually becomes helpful

use std::io::{self, stdout, Write};

use crate::{
    lexer::Lexer,
    token::{token_type::TokenType, Token},
};

pub struct REPL {
    pub version: String,
    pub buffer: String,
    pub index: usize,
    pub is_running: bool,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            version: "v0.1.0".to_string(),
            index: 1,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        // TOOD: make this dynamic, use the manifest from the toml file
        println!(
            "Interactive Eheh ({}) - press Ctrl+C to exit (type h() ENTER for help)",
            "v0.1.0"
        );

        self.is_running = true;

        loop {
            if !self.is_running {
                break;
            }

            self.buffer.clear();
            match self.read() {
                Ok(_) => {
                    self.eval();
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    fn read(&mut self) -> io::Result<()> {
        print!("eheh({})>", self.index);
        stdout().flush()?;
        io::stdin().read_line(&mut self.buffer)?;
        self.index += 1;
        Ok(())
    }

    fn eval(&mut self) {
        if self.consume_command() {
            return;
        }

        let mut lexer = Lexer::new(&self.buffer);

        loop {
            let token: Token = lexer.scan();

            match token.t {
                TokenType::EOF | TokenType::ILLEGAL => break,
                _ => println!("{:?}", token),
            }
        }
    }

    fn consume_command(&mut self) -> bool {
        match self.buffer.as_str().trim_end_matches('\n') {
            "h" | "h()" | "help" | "help()" => {
                println!(
                    r#"
                              Ehelp

Welcome to Interactive Eheh. You are currently seeing the documentation for
the module Ehelp which provides many helpers to make Eheh's shell more
joyful to work with.

This message was triggered by invoking the helper h(), usually referred to as
h/0 (since it expects 0 arguments).

You can use the h/1 function to invoke the documentation for any Eheh module
or function:

    iex> h(Enum)
    iex> h(Enum.map)
    iex> h(Enum.reverse/1)

You can also use the i/1 function to introspect any value you have in the
shell:

    iex> i("hello")

There are many other helpers available, here are some examples:


Help for all of those functions can be consulted directly from the command line
using the h/1 helper itself. Try:

    eheh> h(v/0)

To list all IEx helpers available, which is effectively all exports (functions
and macros) in the IEx.Helpers module:

    eheh> exports(IEx.Helpers)

This module also includes helpers for debugging purposes, see IEx.break!/4 for
more information.

To learn more about IEx as a whole, type h(IEx).

                "#
                );
                true
            }
            "exit" | "exit()" => {
                self.is_running = false;
                true
            }
            _ => false,
        }
    }
}
