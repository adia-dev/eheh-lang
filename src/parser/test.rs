pub mod tests {
    use crate::{
        ast::{
            expressions::{
                boolean_expression::BooleanExpression, call_expression::CallExpression, function_literal::FunctionLiteral,
                identifier::Identifier, if_expression::IfExpression,
                infix_expression::InfixExpression, integer_literal::IntegerLiteral,
                prefix_expression::PrefixExpression,
            },
            statements::{
                declare_statements::DeclareStatement, expression_statements::ExpressionStatement,
                return_statements::ReturnStatement,
            },
        },
        lexer::Lexer,
        parser::Parser,
        token::token_type::{KeywordTokenType, TokenType},
        traits::node::Node,
        types::{ASTExpression, ASTStatement},
    };

    #[test]
    fn test_declare_statements() {
        const CODE: &'static str = r#"
            let x = 0;
            let y;
            const NUMBER_OF_ROWS: i32 = 100;;;
            var notifier;
            let age: u32 = 20 + 3 * 3;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty(), "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 5);

        let expected_identifiers: Vec<(TokenType, &str, &str)> = vec![
            (TokenType::KEYWORD(KeywordTokenType::LET), "x", "0"),
            (TokenType::KEYWORD(KeywordTokenType::LET), "y", ""),
            (
                TokenType::KEYWORD(KeywordTokenType::CONST),
                "NUMBER_OF_ROWS",
                "100",
            ),
            (TokenType::KEYWORD(KeywordTokenType::VAR), "notifier", ""),
            (
                TokenType::KEYWORD(KeywordTokenType::LET),
                "age",
                "(20 + (3 * 3))",
            ),
        ];

        expected_identifiers
            .iter()
            .enumerate()
            .for_each(|(i, identifier)| {
                if let Some(stmt) = program.statements.get(i) {
                    let declare_stmt = downcast_statement_helper::<DeclareStatement>(&stmt);
                    assert_eq!(identifier.0, declare_stmt.token.t);
                    assert_eq!(identifier.1.to_owned(), declare_stmt.name.value);
                    if let Some(value) = &declare_stmt.value {
                        assert_eq!(identifier.2.to_owned(), value.to_string());
                    }
                }
            });
    }

    #[test]
    fn test_identifier_expression() {
        const CODE: &'static str = r#"
            foobar;
            parser;
            lexer;
            program;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let identifiers: Vec<&str> = vec!["foobar", "parser", "lexer", "program"];

        identifiers.iter().enumerate().for_each(|(i, expected)| {
            if let Some(stmt) = program.statements.get(i) {
                let exp_stmt = test_downcast_expression_statement_helper(stmt);
                test_identifier_helper(&exp_stmt.expression, expected.to_string());
            }
        });
    }

    #[test]
    fn test_boolean_expression() {
        const CODE: &'static str = r#"
            true;
            false;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let booleans: Vec<bool> = vec![true, false];

        booleans.iter().enumerate().for_each(|(i, expected)| {
            if let Some(stmt) = program.statements.get(i) {
                let exp_stmt = test_downcast_expression_statement_helper(stmt);
                test_boolean_helper(&exp_stmt.expression, expected.clone());
            }
        });
    }

    #[test]
    fn test_return_statement() {
        const CODE: &'static str = r#"
            return x;
            return y + b;
            return;
            return (10 * 2 * (54 / 2 % 8));
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);
        let expected_values: Vec<&str> = vec![
            "return x;",
            "return (y + b);",
            "return;",
            "return ((10 * 2) * ((54 / 2) % 8));",
        ];

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty(), "{:#?}", parser.errors);
        assert_eq!(program.statements.len(), 4);

        expected_values
            .iter()
            .enumerate()
            .for_each(|(i, expected)| {
                if let Some(stmt) = program.statements.get(i) {
                    let return_exp = downcast_statement_helper::<ReturnStatement>(&stmt);
                    assert_eq!(expected.to_string(), return_exp.to_string());
                }
            });
    }

    #[test]
    fn test_expression_statement_with_identifier() {
        const CODE: &'static str = r#"
            name;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let exp_stmt = test_downcast_expression_statement_helper(&program.statements[0]);
        let ident_exp = downcast_expression_helper::<Identifier>(&exp_stmt.expression);

        assert_eq!(ident_exp.token.t, TokenType::IDENT);
        assert_eq!(ident_exp.get_token_literal(), "name");

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_expression_statement_with_integer_literal() {
        const CODE: &'static str = r#"
            5;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let exp_stmt = test_downcast_expression_statement_helper(&program.statements[0]);
        test_integer_literal_helper(&exp_stmt.expression, 5);

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let mut prefix_inputs: Vec<(&str, &str, &str)> = Vec::new();
        prefix_inputs.push(("!5", "!", "5"));
        prefix_inputs.push(("-5", "-", "5"));
        prefix_inputs.push(("!false", "!", "false"));
        prefix_inputs.push(("!true", "!", "true"));
        prefix_inputs.push(("++i", "++", "i"));
        prefix_inputs.push(("..10", "..", "10"));

        for &(input, operator, value) in &prefix_inputs {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse().unwrap();

            assert_eq!(program.statements.len(), 1);
            assert_eq!(parser.errors.len(), 0);

            let exp_stmt = test_downcast_expression_statement_helper(&program.statements[0]);
            test_prefix_expression_helper(exp_stmt, operator, value);
        }
    }

    #[test]
    fn test_parsing_infix_expressions_with_integer_literals() {
        let mut infix_inputs: Vec<(&str, i64, &str, i64)> = Vec::new();
        infix_inputs.push(("5 + 5;", 5, "+", 5));
        infix_inputs.push(("5 - 5;", 5, "-", 5));
        infix_inputs.push(("5 * 5;", 5, "*", 5));
        infix_inputs.push(("5 % 5;", 5, "%", 5));
        infix_inputs.push(("5 / 5;", 5, "/", 5));
        infix_inputs.push(("5 ^ 5;", 5, "^", 5));

        for &(input, lhs, operator, rhs) in &infix_inputs {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse().unwrap();

            assert_eq!(program.statements.len(), 1);
            assert_eq!(parser.errors.len(), 0);

            let exp_stmt = test_downcast_expression_statement_helper(&program.statements[0]);
            test_infix_expression_helper(exp_stmt, lhs, operator, rhs);
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let mut infix_inputs: Vec<(&str, &str)> = Vec::new();
        infix_inputs.push(("-a * b", "((-a) * b)"));
        // infix_inputs.push(("!-a", "(!(-a))")); -> this is a prefix expression in the end
        infix_inputs.push(("a + b + c", "((a + b) + c)"));
        infix_inputs.push(("a + b - c", "((a + b) - c)"));
        infix_inputs.push(("a * b * c", "((a * b) * c)"));
        infix_inputs.push(("a * b / c", "((a * b) / c)"));
        infix_inputs.push(("a + b / c", "(a + (b / c))"));
        infix_inputs.push(("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"));
        infix_inputs.push(("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"));
        infix_inputs.push(("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"));
        infix_inputs.push(("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"));
        infix_inputs.push(("a * b - t", "((a * b) - t)"));
        infix_inputs.push(("true == false", "(true == false)"));
        infix_inputs.push(("true != false", "(true != false)"));
        infix_inputs.push((
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ));
        infix_inputs.push((
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ));

        for &(input, expected) in &infix_inputs {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse().unwrap();

            for stmt in &program.statements {
                let exp_stmt = test_downcast_expression_statement_helper(stmt);
                downcast_expression_helper::<InfixExpression>(&exp_stmt.expression);
            }

            assert_eq!(program.to_string(), expected);
        }
    }

    #[test]
    fn test_operator_precedence() {
        let mut infix_inputs: Vec<(&str, &str)> = Vec::new();
        infix_inputs.push(("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"));
        infix_inputs.push(("(5 + 5) * 10", "((5 + 5) * 10)"));
        infix_inputs.push(("2 / (5 + 5)", "(2 / (5 + 5))"));
        infix_inputs.push(("-(5 + 5)", "(-(5 + 5))"));
        infix_inputs.push(("!(true == true)", "(!(true == true))"));

        for &(input, expected) in &infix_inputs {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse().unwrap();

            assert!(parser.errors.is_empty(), "{:?}", parser.errors);

            for stmt in &program.statements {
                test_downcast_expression_statement_helper(stmt);
                // downcast_expression_helper::<InfixExpression>(&exp_stmt.expression);
            }

            assert_eq!(program.to_string(), expected);
        }
    }

    #[test]
    fn test_if_expression() {
        const CODE: &'static str = r#"
            if x < (y + 1) { x }
            if x < y { x } else { y }
            if true {
                x // eheh
            }else {
                /* This alternative statement is empty and can be removed.
                 * eheh
                 */
            }
            if false {
                if true {
                    // even nested if statements works !!!!!
                    eheh
                }
            }

            if true {
            } else {
                true
            }

            if true {
            } else {
            }
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty(), "{:#?}", parser.errors);
        // println!("{:#?}", parser.warnings);
        assert_eq!(program.statements.len(), 6);

        for stmt in &program.statements {
            let exp_stmt = test_downcast_expression_statement_helper(stmt);
            let _if_exp = downcast_expression_helper::<IfExpression>(&exp_stmt.expression);
            // println!("{:#?}\n", if_exp);
            // println!("{}\n", if_exp.to_string());
        }
    }

    #[test]
    fn test_function_expression() {
        const CODE: &'static str = r#"
            fn(x, y) { x + y }
            fn add(x, y) { x + y } 
            fn add(x: i32, y: i32) { x + y }
            fn hello_world() {}
            fn hello_user(username: str) -> string {}
            fn join_thread(pid: u32, message: string, callback: fn) { 
                // print("Hello World");
                // ...
                return new_pid;
            }
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty(), "{:#?}", parser.errors);
        // println!("{:#?}", parser.warnings);
        assert_eq!(program.statements.len(), 6);

        for stmt in &program.statements {
            let exp_stmt = test_downcast_expression_statement_helper(stmt);
            let _fn_exp = downcast_expression_helper::<FunctionLiteral>(&exp_stmt.expression);
            // println!("{}\n", exp_stmt.to_string());
        }
    }

    #[test]
    fn test_call_expression() {
        const CODE: &'static str = r#"
            add(1, 2)
            print(1 + 1, 3 * 2 / 3)
            start_server()
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty(), "{:#?}", parser.errors);
        // println!("{:#?}", parser.warnings);
        assert_eq!(program.statements.len(), 3);

        for stmt in &program.statements {
            let exp_stmt = test_downcast_expression_statement_helper(stmt);
            let _call_exp = downcast_expression_helper::<CallExpression>(&exp_stmt.expression);
            // println!("{}\n", call_exp.to_string());
        }
    }

    fn test_downcast_expression_statement_helper(statement: &ASTStatement) -> &ExpressionStatement {
        match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(exp_stmt) => exp_stmt,
            None => {
                panic!("Could not downcast_ref a statement into an ExpressionStatement.")
            }
        }
    }

    fn downcast_expression_helper<T: 'static>(exp: &ASTExpression) -> &T {
        match exp.as_any().downcast_ref::<T>() {
            Some(t_exp) => t_exp,
            None => {
                panic!(
                    "Failed to downcast an expression: {:?}",
                    exp.get_token_literal()
                )
            }
        }
    }

    fn downcast_statement_helper<T: 'static>(stmt: &ASTStatement) -> &T {
        match stmt.as_any().downcast_ref::<T>() {
            Some(t_stmt) => t_stmt,
            None => {
                panic!("Failed to downcast an expression")
            }
        }
    }

    fn test_infix_expression_helper(
        exp_stmt: &ExpressionStatement,
        lhs: i64,
        operator: &str,
        rhs: i64,
    ) {
        let infix_exp = downcast_expression_helper::<InfixExpression>(&exp_stmt.expression);
        let lhs_exp = test_integer_literal_helper(&infix_exp.lhs, lhs);
        let rhs_exp = test_integer_literal_helper(&infix_exp.rhs, rhs);

        assert_eq!(lhs, lhs_exp.value);
        assert_eq!(operator, infix_exp.operator);
        assert_eq!(rhs, rhs_exp.value);
    }

    fn test_prefix_expression_helper(exp_stmt: &ExpressionStatement, operator: &str, rhs: &str) {
        let prefix_exp = downcast_expression_helper::<PrefixExpression>(&exp_stmt.expression);
        assert_eq!(operator, prefix_exp.operator);
        assert_eq!(rhs, prefix_exp.rhs.to_string());
    }

    fn test_identifier_helper(exp: &ASTExpression, value: String) -> &Identifier {
        let ident = downcast_expression_helper::<Identifier>(exp);
        assert_eq!(ident.value, value);
        assert_eq!(ident.get_token_literal(), value);
        ident
    }

    fn test_boolean_helper(exp: &ASTExpression, value: bool) -> &BooleanExpression {
        let boolean = downcast_expression_helper::<BooleanExpression>(exp);
        assert_eq!(boolean.value, value);
        assert_eq!(boolean.get_token_literal(), value.to_string());
        boolean
    }

    fn test_integer_literal_helper(exp: &ASTExpression, expected_value: i64) -> &IntegerLiteral {
        match exp.as_any().downcast_ref::<IntegerLiteral>() {
            Some(integer_literal) => {
                assert_eq!(
                    integer_literal.value, expected_value,
                    "Expected IntegerLiteral expression value to be {}, got {} instead.",
                    expected_value, integer_literal.value
                );
                assert_eq!(
                    integer_literal.get_token_literal(),
                    expected_value.to_string(),
                    "Expected IntegerLiteral expression token literal to be \"{}\", got \"{}\" instead.",
                    expected_value.to_string(),
                    integer_literal.get_token_literal()
                );
                integer_literal
            }
            None => {
                panic!("Expected IntegerLiteral expression, got {:?}", exp)
            }
        }
    }
}
