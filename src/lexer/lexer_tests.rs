mod tests {

    use crate::{
        lexer::Lexer,
        token::token_type::{
            KeywordTokenType::{self},
            TokenType,
        },
    };

    #[test]
    fn test_scan() {
        const CODE: &'static str = "=+(){}[],:;";

        let mut expected_tokens: Vec<(TokenType, String)> = Vec::new();
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::LBRACK, "[".to_owned()));
        expected_tokens.push((TokenType::RBRACK, "]".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::COLON, ":".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::EOF, "\0".to_owned()));

        let mut lexer = Lexer::new(CODE.as_ref());

        for (t, literal) in expected_tokens {
            let token = lexer.scan();

            assert_eq!(
                token.t, t,
                "Expected token {} got {} at {}:{}",
                t, token.t, token.line, token.position
            );
            assert_eq!(
                token.t, t,
                "Expected literal {} got {} at {}:{}",
                literal, token.literal, token.line, token.position
            );
        }
    }

    #[test]
    fn test_scan_with_code() {
        const CODE: &'static str = r#"
        let five = 5;
        const ten = 10;

        let add = fn(x, y) {
            x + y
        };

        let result = add(five, ten);
    "#;

        let mut expected_tokens: Vec<(TokenType, String)> = Vec::new();
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "5".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::CONST),
            "const".to_owned(),
        ));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "10".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::FUN), "fn".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "result".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::EOF, "\0".to_owned()));

        let mut lexer = Lexer::new(CODE.as_ref());

        for (t, literal) in expected_tokens {
            let token = lexer.scan();

            assert_eq!(
                token.t, t,
                "Expected token {} got {} at {}:{}",
                t, token.t, token.line, token.position
            );
            assert_eq!(
                token.t, t,
                "Expected literal {} got {} at {}:{}",
                literal, token.literal, token.line, token.position
            );
        }
    }

    #[test]
    fn test_lexer_with_various_tokens() {
        const CODE: &'static str = r#"
            let five = 5;
            const ten = 10;
    
            let add = fn(x, y) {
                x + y
            };
    
            let result = add(five, ten);
    
            let boolean_true = true;
            let boolean_false = false;
    
            let string_literal = "Hello, World!";
    
            let bitwise_and = 3 & 7;
            let bitwise_or = 1 | 2;
            let bitwise_xor = 6 ^ 4;
    
            let negation = !boolean_true;
    
            let plus = 1 + 2;
            let minus = 4 - 3;
            let multiply = 5 * 6;
            let divide = 10 / 2;
    
            let comparison_gt = 7 > 3;
            let comparison_lt = 2 < 5;
            let comparison_gte = 8 >= 8;
            let comparison_lte = 6 <= 6;
            let comparison_eq = 1 == 1;
            let comparison_neq = 10 != 3;
    
            let and_keyword = if true && false { 1 } else { 0 };
            let or_keyword = if true || false { 1 } else { 0 };
    
            let array = [1, 2, 3];
            let index_access = array[0];
    
            let object = { "key": "value", "age": 25 };
            let object_access = object["key"];
            let object_method = object.some_method();
    
            let comment = // This is a comment
                          /* This is another comment */;
    
            let null_value = null;
        "#;

        let mut expected_tokens: Vec<(TokenType, String)> = Vec::new();
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "5".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::CONST),
            "const".to_owned(),
        ));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "10".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::FUN), "fn".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "result".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "boolean_true".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::TRUE),
            "true".to_owned(),
        ));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "boolean_false".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::FALSE),
            "false".to_owned(),
        ));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "string_literal".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::STRING, "Hello, World!".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "bitwise_and".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "3".to_owned()));
        expected_tokens.push((TokenType::AMPERSAND, "&".to_owned()));
        expected_tokens.push((TokenType::INT, "7".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "bitwise_or".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::PIPE, "|".to_owned()));
        expected_tokens.push((TokenType::INT, "2".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "bitwise_xor".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "6".to_owned()));
        expected_tokens.push((TokenType::EXPONENT, "^".to_owned()));
        expected_tokens.push((TokenType::INT, "4".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "negation".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::BANG, "!".to_owned()));
        expected_tokens.push((TokenType::IDENT, "boolean_true".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "plus".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::INT, "2".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "minus".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "4".to_owned()));
        expected_tokens.push((TokenType::MINUS, "-".to_owned()));
        expected_tokens.push((TokenType::INT, "3".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "multiply".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "5".to_owned()));
        expected_tokens.push((TokenType::ASTERISK, "*".to_owned()));
        expected_tokens.push((TokenType::INT, "6".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "divide".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "10".to_owned()));
        expected_tokens.push((TokenType::FORWARDSLASH, "/".to_owned()));
        expected_tokens.push((TokenType::INT, "2".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_gt".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "7".to_owned()));
        expected_tokens.push((TokenType::GT, ">".to_owned()));
        expected_tokens.push((TokenType::INT, "3".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_lt".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "2".to_owned()));
        expected_tokens.push((TokenType::LT, "<".to_owned()));
        expected_tokens.push((TokenType::INT, "5".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_gte".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "8".to_owned()));
        expected_tokens.push((TokenType::GTE, ">=".to_owned()));
        expected_tokens.push((TokenType::INT, "8".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_lte".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "6".to_owned()));
        expected_tokens.push((TokenType::LTE, "<=".to_owned()));
        expected_tokens.push((TokenType::INT, "6".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_eq".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::EQ, "==".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comparison_neq".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "10".to_owned()));
        expected_tokens.push((TokenType::NEQ, "!=".to_owned()));
        expected_tokens.push((TokenType::INT, "3".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "and_keyword".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::IF), "if".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::TRUE),
            "true".to_owned(),
        ));
        expected_tokens.push((TokenType::AND, "&&".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::FALSE),
            "false".to_owned(),
        ));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::ELSE),
            "else".to_owned(),
        ));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::INT, "0".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "or_keyword".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::IF), "if".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::TRUE),
            "true".to_owned(),
        ));
        expected_tokens.push((TokenType::OR, "||".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::FALSE),
            "false".to_owned(),
        ));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::ELSE),
            "else".to_owned(),
        ));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::INT, "0".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "array".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::LBRACK, "[".to_owned()));
        expected_tokens.push((TokenType::INT, "1".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::INT, "2".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::INT, "3".to_owned()));
        expected_tokens.push((TokenType::RBRACK, "]".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "index_access".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "array".to_owned()));
        expected_tokens.push((TokenType::LBRACK, "[".to_owned()));
        expected_tokens.push((TokenType::INT, "0".to_owned()));
        expected_tokens.push((TokenType::RBRACK, "]".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "object".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::STRING, "key".to_owned()));
        expected_tokens.push((TokenType::COLON, ":".to_owned()));
        expected_tokens.push((TokenType::STRING, "value".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::STRING, "age".to_owned()));
        expected_tokens.push((TokenType::COLON, ":".to_owned()));
        expected_tokens.push((TokenType::INT, "25".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "object_access".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "object".to_owned()));
        expected_tokens.push((TokenType::LBRACK, "[".to_owned()));
        expected_tokens.push((TokenType::STRING, "key".to_owned()));
        expected_tokens.push((TokenType::RBRACK, "]".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "object_method".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "object".to_owned()));
        expected_tokens.push((TokenType::DOT, ".".to_owned()));
        expected_tokens.push((TokenType::IDENT, "some_method".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "comment".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "null_value".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::NULL),
            "null".to_owned(),
        ));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        let mut lexer = Lexer::new(CODE.as_ref());

        for (t, literal) in expected_tokens {
            let token = lexer.scan();

            // println!("{:?}", token);

            assert_eq!(
                token.t, t,
                "Expected token {} got {} at {}:{}",
                t, token.t, token.line, token.position
            );
            assert_eq!(
                token.literal, literal,
                "Expected literal {} got {} at {}:{}",
                literal, token.literal, token.line, token.position
            );
        }
    }
}
