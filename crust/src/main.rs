mod ast;

use crate::ast::lexer::Lexer;
use crate::ast::Ast;
use crate::ast::parser::Parser;

fn main() {
    // Test binary expression (e.g. 7 + 8) + order of precedence (e.g. PEMDAS)
    let input = "7 + 8 * 9";
    // Previous test input: 7 + 3 * (10 / (12 / (3 + 1) - 1))

    // Part I: Lexer
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    // Part II: Parser
    let mut ast: Ast = Ast::new();
    let mut parser = Parser::from_tokens(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();
}
