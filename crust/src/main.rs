mod ast;
mod diagnostics;

use crate::ast::lexer::Lexer;
use crate::ast::Ast;
use crate::ast::parser::Parser;
use crate::ast::evaluator::ASTEvaluator;

fn main() {
    let input = "7 - (30 + 7) * 8 & 2";

    // Part I: Lexer
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    let diagnostics_bag: DiagnosticsBagCell = Rc::new(
        RefCell::new(diagnostics::DiagnosticsBag::new())
    );

    // Part II: Parser
    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    // Visualizer
    ast.visualize();

    // Evaluator
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Result: {:?}", eval.last_value);
}
