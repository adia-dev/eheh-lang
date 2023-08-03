pub mod tests {
    use crate::{
        ast::{
            expressions::{
                identifier::Identifier, infix_expression::InfixExpression,
                integer_literal::IntegerLiteral, prefix_expression::PrefixExpression,
            },
            statements::{
                declare_statements::DeclareStatement, expression_statements::ExpressionStatement,
            },
        },
        lexer::Lexer,
        parser::Parser,
        token::token_type::TokenType,
        traits::node::Node,
    };

    #[test]
    fn test_declare_statements() {
        const CODE: &'static str = r#"
            let x = 0;
            let y;
            const NUMBER_OF_ROWS: i32 = 100;;;
            var notifier = null;
            let first_name: string = "Abdoulaye Dia";
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 5);

        let expected_identifiers: Vec<&str> =
            vec!["x", "y", "NUMBER_OF_ROWS", "notifier", "first_name"];

        expected_identifiers
            .iter()
            .enumerate()
            .for_each(|(i, identifier)| {
                if let Some(stmt) = program.statements.get(i) {
                    match stmt.as_any().downcast_ref::<DeclareStatement>() {
                        Some(declare_stmt) => {
                            assert_eq!(identifier.to_owned(), declare_stmt.name.value);
                        }
                        _ => panic!("Parsing Error: Expected to receive a Declare Statement."),
                    }
                }
            });
    }

    #[test]
    fn test_return_statement() {
        const CODE: &'static str = r#"
            return x;
            return y + b;
            return;
            return ("Abdoulaye Dia");
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_expression_statement_with_identifier() {
        const CODE: &'static str = r#"
            name;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let exp_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let exp_ident = exp_stmt
            .expression
            .as_any()
            .downcast_ref::<Identifier>()
            .unwrap();

        assert_eq!(exp_ident.token.t, TokenType::IDENT);
        assert_eq!(exp_ident.get_token_literal(), "name");

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

        let exp_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();

        let exp_int = exp_stmt
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .unwrap();

        assert_eq!(exp_int.token.t, TokenType::INT);
        assert_eq!(exp_int.get_token_literal(), "5");
        assert_eq!(exp_int.value, 5);

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let mut prefix_inputs: Vec<(&str, &str, &str)> = Vec::new();
        prefix_inputs.push(("!5", "!", "5"));
        prefix_inputs.push(("-5", "-", "5"));
        prefix_inputs.push(("++i", "++", "i"));
        prefix_inputs.push(("..10", "..", "10"));

        for &(input, operator, value) in &prefix_inputs {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse().unwrap();

            assert_eq!(program.statements.len(), 1);
            assert_eq!(parser.errors.len(), 0);

            if let Some(stmt) = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                let prefix_exp = stmt
                    .expression
                    .as_any()
                    .downcast_ref::<PrefixExpression>()
                    .unwrap();

                assert_eq!(operator, prefix_exp.operator);
                assert_eq!(value, prefix_exp.rhs.to_string());
            } else {
                panic!("Could not perform the downcast into an Expression Statement")
            }
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

            if let Some(stmt) = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
            {
                let infix_exp = stmt
                    .expression
                    .as_any()
                    .downcast_ref::<InfixExpression>()
                    .unwrap();

                let lhs_exp = infix_exp
                    .lhs
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .unwrap();

                let rhs_exp = infix_exp
                    .rhs
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .unwrap();

                assert_eq!(lhs, lhs_exp.value);
                assert_eq!(operator, infix_exp.operator);
                assert_eq!(rhs, rhs_exp.value);
            } else {
                panic!("Could not perform the downcast into an Expression Statement")
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let mut infix_inputs: Vec<(&str, &str)> = Vec::new();
        infix_inputs.push(("-a * b", "((-a) * b)"));
        infix_inputs.push(("!-a", "(!(-a))"));
        infix_inputs.push(("a + b + c", "((a + b) + c)"));
        infix_inputs.push(("a + b - c", "((a + b) - c)"));
        infix_inputs.push(("a * b * c", "((a * b) * c)"));
        infix_inputs.push(("a * b / c", "((a * b) / c)"));
        infix_inputs.push(("a + b / c", "(a + (b / c))"));
        infix_inputs.push(("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"));
        infix_inputs.push(("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"));
        infix_inputs.push(("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"));
        infix_inputs.push(("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"));
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

            assert_eq!(program.to_string(), expected);
        }
    }
}
