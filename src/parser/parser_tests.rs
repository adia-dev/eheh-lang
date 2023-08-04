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
        token::token_type::{KeywordTokenType, TokenType},
        traits::{expression::Expression, node::Node, statement::Statement},
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

        assert!(parser.errors.is_empty(), "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 5);

        let expected_identifiers: Vec<(TokenType, &str)> = vec![
            (TokenType::KEYWORD(KeywordTokenType::LET), "x"),
            (TokenType::KEYWORD(KeywordTokenType::LET), "y"),
            (
                TokenType::KEYWORD(KeywordTokenType::CONST),
                "NUMBER_OF_ROWS",
            ),
            (TokenType::KEYWORD(KeywordTokenType::VAR), "notifier"),
            (TokenType::KEYWORD(KeywordTokenType::LET), "first_name"),
        ];

        expected_identifiers
            .iter()
            .enumerate()
            .for_each(|(i, identifier)| {
                if let Some(stmt) = program.statements.get(i) {
                    let declare_stmt = downcast_statement_helper::<DeclareStatement>(&stmt);
                    assert_eq!(identifier.0, declare_stmt.token.t);
                    assert_eq!(identifier.1.to_owned(), declare_stmt.name.value);
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

    fn test_downcast_expression_statement_helper(
        statement: &Box<dyn Statement>,
    ) -> &ExpressionStatement {
        match statement.as_any().downcast_ref::<ExpressionStatement>() {
            Some(exp_stmt) => exp_stmt,
            None => {
                panic!("Could not downcast_ref a statement into an ExpressionStatement.")
            }
        }
    }

    fn downcast_expression_helper<T: 'static>(exp: &Box<dyn Expression>) -> &T {
        match exp.as_any().downcast_ref::<T>() {
            Some(t_exp) => t_exp,
            None => {
                panic!("Failed to downcast an expression")
            }
        }
    }

    fn downcast_statement_helper<T: 'static>(stmt: &Box<dyn Statement>) -> &T {
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
    ) 
    {
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

    fn test_identifier_helper(exp: &Box<dyn Expression>, value: String) -> &Identifier {
        let ident = downcast_expression_helper::<Identifier>(exp);
        assert_eq!(ident.value, value);
        assert_eq!(ident.get_token_literal(), value);
        ident
    }

    fn test_integer_literal_helper(
        exp: &Box<dyn Expression>,
        expected_value: i64,
    ) -> &IntegerLiteral {
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
