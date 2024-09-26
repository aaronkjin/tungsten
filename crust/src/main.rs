use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::Ast;
use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::ASTLetStatement;
use crate::ast::ASTVisitor;
use crate::diagnostics::DiagnosticsBagCell;
use crate::diagnostics::printer::DiagnosticsPrinter;

mod ast;
mod diagnostics;
mod text;

struct SymbolChecker {
    symbols: Vec<String>,
}

impl ASTVisitor for SymbolChecker {
    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.symbols.push(let_statement.identifier.span.literal.clone());
    }
}

fn main() {
    let input =
        "
        let a = 10 + 30
        let b = 20
        let c = 10 + e
        let d = (a + b) * c 
    ";
    let text = text::SourceText::new(input.to_string());

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
    let mut parser = Parser::new(tokens, Rc::clone(&diagnostics_bag));

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    // Visualizer
    ast.visualize();

    // Diagnostics printer
    let diagnostics_binding = diagnostics_bag.borrow();
    if diagnostics_binding.diagnostics.len() > 0 {
        let diagnostics_printer = DiagnosticsPrinter::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
        return;
    }

    // Evaluator
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Result: {:?}", eval.last_value);
}
