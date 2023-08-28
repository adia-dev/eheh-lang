use std::{error::Error, fmt::Display};

use colored::Colorize;

use crate::token::{token_type::TokenType, Token};

#[derive(Debug, Clone)]
#[repr(usize)]
pub enum ParserErrorCode {
    UnexpectedToken {
        token: Token,
        expected_token_types: Vec<TokenType>,
        context: Option<String>,
    } = 1_000,
    UnknownPrefixToken {
        token: Token,
        context: Option<String>,
    },
    MissingIfCondition {
        token: Token,
        context: Option<String>,
    },
    DelimiterMismatch {
        token: Token,
        expected_delimiter: TokenType,
        current_delimiter: Option<TokenType>,
        context: Option<Vec<String>>,
    },
    MissingFnReturnType {
        token: Token,
        context: Option<String>,
    },
    Unknown,
}

impl ParserErrorCode {
    pub fn id(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

impl Display for ParserErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorCode::UnexpectedToken {
                token,
                expected_token_types,
                context,
            } => {
                write!(f, "{}", format!("error[E{:0>5}]", self.id()).red())?;

                match expected_token_types.len() {
                    0 => (),
                    1 => {
                        write!(f, ": expected `{}`", expected_token_types[0].to_literal())?;
                    }
                    _ => {
                        write!(
                            f,
                            ": expected one of {} or `{}`",
                            expected_token_types[0..expected_token_types.len() - 1]
                                .iter()
                                .map(|t| { format!("`{}`", t.to_literal()) })
                                .collect::<Vec<String>>()
                                .join(", "),
                            expected_token_types.last().unwrap().to_literal()
                        )?;
                    }
                }

                writeln!(f, ", found `{}`", token.literal)?;
                writeln!(f, "  {} {}", "-->".blue(), token.get_location())?;

                if let Some(ctx) = context {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        ctx
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).red()
                    )?;
                    match expected_token_types.len() {
                        1 => {
                            writeln!(f, ": expected `{}`", expected_token_types[0].to_literal())?;
                        }
                        _ => {
                            write!(
                                f,
                                ": expected one of {}",
                                expected_token_types[0..expected_token_types.len() - 1]
                                    .iter()
                                    .map(|t| { format!("`{}`", t.to_literal()) })
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            )?;

                            writeln!(
                                f,
                                " or `{}`",
                                expected_token_types.last().unwrap().to_literal()
                            )?;
                        }
                    };
                }
            }
            ParserErrorCode::MissingIfCondition { token, context } => {
                writeln!(
                    f,
                    "{}: The `if` expression is missing a condition.",
                    format!("error[E{:0>5}]", self.id()).red()
                )?;

                writeln!(f, "  {} {}", "-->".blue(), token.get_location())?;

                if let Some(ctx) = context {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        ctx
                    )?;
                    writeln!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).red()
                    )?;
                };
            }
            ParserErrorCode::MissingFnReturnType { token, context } => {
                writeln!(
                    f,
                    "{}: The function is missing a return type.",
                    format!("error[E{:0>5}]", self.id()).red()
                )?;

                writeln!(f, "  {} {}", "-->".blue(), token.get_location())?;

                if let Some(ctx) = context {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        ctx
                    )?;
                    writeln!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).red()
                    )?;
                }
            }
            ParserErrorCode::Unknown => {
                writeln!(
                    f,
                    "{}: An unknown error occured while parsing the code.",
                    format!("error[E{:0>5}]", self.id()).red(),
                );
            }
            ParserErrorCode::UnknownPrefixToken { token, context } => {
                writeln!(
                    f,
                    "{}: Unknown prefix token: `{}`.",
                    format!("error[E{:0>5}]", self.id()).red(),
                    token.literal
                )?;
                writeln!(f, "  {} {}", "-->".blue(), token.get_location())?;

                if let Some(ctx) = context {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        ctx
                    )?;
                    writeln!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).red()
                    )?;
                }
            }
            ParserErrorCode::DelimiterMismatch {
                token,
                expected_delimiter,
                current_delimiter,
                context,
            } => {
                write!(
                    f,
                    "{}: The `{}` delimiter is missing its pair, expected to match with a `{}`",
                    format!("error[E{:0>5}]", self.id()).red(),
                    TokenType::matching_delimiter(expected_delimiter.to_owned())
                        .unwrap()
                        .to_literal(),
                    expected_delimiter.to_literal()
                )?;

                if let Some(delimiter) = current_delimiter {
                    writeln!(f, ", got {}.", delimiter.to_literal())?;
                } else {
                    writeln!(f, ", got {}.", token.t.to_literal())?;
                }

                writeln!(f, "  {} {}", "-->".blue(), token.get_location())?;

                if let Some(ctx) = context {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        ctx.join("\n")
                    )?;
                    writeln!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).red()
                    )?;
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub source: Option<Box<ParserError>>,
    pub code: ParserErrorCode,
}

impl ParserError {
    pub fn set_source(&mut self, err: ParserError) {
        self.source = Some(Box::new(err));
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(sauce) = &self.source {
            Some(sauce)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_various_errors() {
        const CODE: &'static str = r#"
            fn x: number) -> void {
                x * 2
            }

            fn () -> {}

            if (true
            {
                
            }
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);
        let _program = parser.parse().unwrap();

        // unexpected token: `x` -> `(`
        // Unknown prefix token
        // Delimiter mismatch (true -> missing `)`
        // Unknown prefix token
        let error_codes: Vec<usize> = vec![1000, 1001, 1004, 1000, 1001];
        for (index, code) in error_codes.iter().enumerate() {
            assert_eq!(parser.errors[index].code.id(), *code);
        }
    }
}
