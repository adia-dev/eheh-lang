use std::{
    cell::RefCell,
    io::{self, stdout, Write},
    rc::Rc,
};

use crate::{
    evaluator::Evaluator,
    lexer::Lexer,
    objects::environment::Environment,
    parser::Parser,
    traits::{node::Node, object::ObjectType},
};

pub struct REPL {
    pub version: String,
    pub buffer: String,
    pub index: usize,
    pub is_running: bool,
    pub environment: Rc<RefCell<Environment>>,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            version: "v0.1.0".to_string(),
            index: 1,
            is_running: false,
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn start(&mut self) {
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
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        if !parser.errors.is_empty() {
            for error in &parser.errors {
                print!("{}", error);
            }
            return;
        }

        for warning in &parser.warnings {
            println!("{}", warning);
        }

        let evaluated =
            Evaluator::eval(Box::new(program.as_node()), self.environment.clone()).unwrap();

        if evaluated.t() != ObjectType::Null {
            println!("{}\n", evaluated.to_string());
        }
    }

    fn consume_command(&mut self) -> bool {
        match self.buffer.as_str().trim_end_matches('\n') {
            "h" | "h()" | "help" | "help()" => {
                println!("TODO: some actual help....");
            }
            "env" | "env()" | "environment" | "environment()" => {
                self.print_environment();
            }
            "exit" | "exit()" => {
                println!("~ByeBye~");
                self.is_running = false;
            }
            _ => return false,
        }

        true
    }

    fn print_environment(&self) {
        println!("Environment:");
        if self.environment.borrow().store.is_empty() {
            println!("(empty)");
            return;
        }
        for (key, object) in self.environment.borrow().store.iter() {
            println!("    - {}: {}", key, object.borrow().to_string())
        }
    }
}
