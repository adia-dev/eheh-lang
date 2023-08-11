#![allow(dead_code)]
#![allow(unused)]

use std::any::type_name;

use colored::Colorize;
use rlpl::RLPL;

use crate::token::{Token, token_type::TokenType};

mod ast;
// mod error;
mod lexer;
mod parser;
// mod error;
mod program;
mod repl;
mod rlpl;
mod token;
mod traits;
mod types;

fn main() {
    // let mut repl = REPL::new();

    // repl.start();

    // let mut rlpl = RLPL::new();

    // rlpl.start();

    let token = Token::new(TokenType::RPAREN, ")".to_string(), 10, 34);

    let type_name = "ASTExpression";
    let file_name = "src/ast/expressions/prefix_expression.rs:10:14";
    let file_name_2 = "src/traits/expression.rs:3:1";
    let ctx_type = "type";
    let ctx = "pub rhs: ASTExpression,";
    let trait_ctx = "pub trait Expression: Node {";
    let trait_suggestion = "similarly named trait `Expression` defined here";
    let ctx_fix = "pub rhs: Expression,".replace("Expression", "Expression".green().to_string().as_str());
    let ctx_fix_name = "Expression";
    let ctx_name = "ASTExpression";
    let help = "a trait with a similar name exists";
    let help_2 = "consider importing this type alias";
    let ctx_suggestion = "use crate::types::ASTExpression;";

    let ctx_start_pos = ctx.find(ctx_name).unwrap();


    print!("{}", "error[E0412]".red());
    println!(": cannot find {} `{}` in this scope", ctx_type, ctx_name);
    println!("  {} {}", "-->".blue(), file_name);
    println!("   {}", "|".blue());
    println!("{:2} {}\t{}", 10.to_string().blue(), "|".blue(), ctx);
    println!("   {}\t{}{}", "|".blue(), " ".repeat(ctx_start_pos), "^".repeat(ctx_name.len()).red());
    println!("   {}", "|".blue());

    println!(" {} {}", "[:::]".blue(), file_name_2);

    println!("   {}", "|".blue());
    println!("{:2} {}\t{}", 3.to_string().blue(), "|".blue(), trait_ctx);
    println!("   {}\t{} {}", "|".blue(), "-".repeat(trait_ctx.len() - 1).blue(), trait_suggestion.blue());
    println!("   {}", "|".blue());
    println!("{}: {}", "help".cyan(), help);
    println!("   {}", "|".blue());
    println!("{:2} {}\t{}", 10.to_string().blue(), "|".blue(), ctx_fix);
    println!("   {}\t{}{}", "|".blue(), " ".repeat(ctx_start_pos), "~".repeat(ctx_fix_name.len()).green());
    println!("{}: {}", "help".cyan(), help_2);
    println!("   {}", "|".blue());
    println!("{:2} {}\t{}", 1.to_string().blue(), "|".blue(), ctx_suggestion.green());
    println!("   {}", "|".blue());
    println!("");

    print!("{}", "warning[W0412]".yellow());
    println!(": unused import: `{}` in this scope", "repl::REPL");
    println!("  {} {}", "-->".blue(), file_name);
    println!("   {}", "|".blue());
    println!("{:2} {}\t{}", 10.to_string().blue(), "|".blue(), "use repl::REPL;");
    println!("   {}\t{}{}", "|".blue(), " ".repeat(4), "^".repeat("repl::REPL".len()).yellow());
    println!("   {}", "|".blue());
    println!("   {} policy: @UnusedVariables: On", "->".blue());
    println!("");
}
