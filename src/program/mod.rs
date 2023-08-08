use crate::{traits::node::Node, types::ASTStatement};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<ASTStatement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn get_token_literal(&self) -> String {
        if self.statements.is_empty() {
            "".to_string()
        } else {
            self.statements[0].get_token_literal()
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(statement.to_string().as_str());
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_program_to_string() {
        const CODE: &'static str = r#"
            let x = 0;
            let y;
            const NUMBER_OF_ROWS: i32 = 100;
            var notifier = 10;
            let age: u32 = 20 + 3;
            return 10;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 6);

        let expected_strings: Vec<&str> = vec![
            "let x = 0;",
            "let y;",
            "const NUMBER_OF_ROWS: i32 = 100;",
            "var notifier = 10;",
            "let age: u32 = (20 + 3);",
            "return 10;",
        ];

        expected_strings.iter().enumerate().for_each(|(i, s)| {
            if let Some(stmt) = program.statements.get(i) {
                assert_eq!(stmt.to_string(), s.to_string());
            }
        });
    }
}
