use std::{error::Error, fmt::Display};

use colored::Colorize;

use crate::token::{token_type::TokenType, Token};

#[derive(Debug)]
#[repr(usize)]
pub enum ParserWarning {
    UnnecessaryParentheses {
        token: Token,
        around: Token,
        context: Vec<String>,
    } = 1_000,
    Unknown,
}

impl ParserWarning {
    fn id(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

impl Display for ParserWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserWarning::UnnecessaryParentheses {
                token,
                context,
                around,
            } => {
                writeln!(
                    f,
                    "{} unnecessary parentheses around {}",
                    format!("warning[E{:0>5}]", self.id()).yellow(),
                    around.literal
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 2),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::Unknown => todo!(),
        }
    }
}
