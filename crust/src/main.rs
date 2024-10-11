use crate::ast::Ast;
use crate::ast::ASTBinaryExpression;
use crate::ast::ASTLetStatement;
use crate::ast::ASTNumberExpression;
use crate::ast::ASTParenthesizedExpression;
use crate::ast::ASTVariableExpression;
use crate::ast::ASTVisitor;
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::lexer::Lexer;
use crate::ast::lexer::TextSpan;
use crate::ast::parser::Parser;
use crate::diagnostics::DiagnosticsBagCell;
use crate::diagnostics::printer::DiagnosticsPrinter;
use crate::text::SourceText;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod ast;
mod diagnostics;
mod text;

struct SymbolChecker {
    symbols: HashMap<String, ()>,
    diagnostics: DiagnosticsBagCell,
}

impl SymbolChecker {
    fn new(diagnostics: DiagnosticsBagCell) -> Self {
        SymbolChecker {
            symbols: HashMap::new(),
            diagnostics,
        }
    }
}

impl ASTVisitor for SymbolChecker {
    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        let identifier = let_statement.identifier.span.literal.clone();
        self.visit_expression(&let_statement.initializer);
        self.symbols.insert(identifier, ());
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        if !self.symbols.contains_key(&variable_expression.identifier.span.literal) {
            let mut diagnostics_binding = self.diagnostics.borrow_mut();
            diagnostics_binding.report_undeclared_variable(&variable_expression.identifier);
        }
    }

    // TODO: fix visit error
    fn visit_error(&mut self, _span: &TextSpan) {}

    fn visit_number_expression(&mut self, _number: &ASTNumberExpression) {}

    fn visit_binary_expression(&mut self, _binary_expression: &ASTBinaryExpression) {}

    fn visit_parenthesized_expression(
        &mut self,
        _parenthesized_expression: &ASTParenthesizedExpression
    ) {}
}

fn main() -> Result<(), ()> {
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
    check_diagnostics(&text, &diagnostics_bag)?;
    let mut symbol_checker = SymbolChecker::new(Rc::clone(&diagnostics_bag));
    ast.visit(&mut symbol_checker);
    check_diagnostics(&text, &diagnostics_bag)?;

    // Evaluator
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("Result: {:?}", eval.last_value);

    Ok(())
}

fn check_diagnostics(text: &SourceText, diagnostics_bag: &DiagnosticsBagCell) -> Result<(), ()> {
    let diagnostics_binding = diagnostics_bag.borrow();

    if diagnostics_binding.diagnostics.len() > 0 {
        let diagnostics_printer = DiagnosticsPrinter::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
        return Err(());
    }

    Ok(())
}
