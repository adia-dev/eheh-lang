use std::io::{self, Write, stdout};

use crate::{
    lexer::Lexer,
    token::{token_type::TokenType, Token},
};

pub struct REPL {
    pub version: String,
    pub buffer: String,
    pub index: usize,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            version: "v0.1.0".to_string(),
            index: 1,
        }
    }

    pub fn start(&mut self) {
        loop {
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
        print!("({})>", self.index);
        stdout().flush()?;
        io::stdin().read_line(&mut self.buffer)?;
        self.index += 1;
        Ok(())
    }

    fn eval(&mut self) {
        let mut lexer = Lexer::new(&self.buffer);

        loop {
            let token: Token = lexer.scan();

            match token.t {
                TokenType::EOF | TokenType::ILLEGAL => break,
                _ => println!("{:?}", token),
            }
        }
    }
}
