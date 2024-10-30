use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::{
    Ast,
    ASTLetStatement,
    ASTNumberExpression,
    ASTUnaryExpression,
    ASTVariableExpression,
    ASTVisitor,
};
use crate::ast::lexer::{ Lexer, TextSpan };
use crate::diagnostics::DiagnosticsBagCell;
use crate::{ diagnostics, text };
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::parser::Parser;
use crate::diagnostics::printer::DiagnosticsPrinter;

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
        if self.symbols.get(&variable_expression.identifier.span.literal).is_none() {
            let mut diagnostics_binding = self.diagnostics.borrow_mut();

            diagnostics_binding.report_undeclared_variable(&variable_expression.identifier);
        }
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {}

    fn visit_error(&mut self, span: &TextSpan) {}
}

pub struct CompilationUnit {
    ast: Ast,
    diagnostics_bag: DiagnosticsBagCell,
}

impl CompilationUnit {
    pub fn compile(input: &str) -> Result<CompilationUnit, CompilationUnit> {
        let text = text::SourceText::new(input.to_string());

        // Part I: Lexer
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        let diagnostics_bag: DiagnosticsBagCell = Rc::new(
            RefCell::new(diagnostics::DiagnosticsBag::new())
        );

        // Part II: Parser
        let mut ast: Ast = Ast::new();
        let mut parser = Parser::new(tokens, Rc::clone(&diagnostics_bag));

        while let Some(stmt) = parser.next_statement() {
            ast.add_statement(stmt);
        }

        // Visualizer
        ast.visualize();

        // Diagnostics printer
        Self::check_diagnostics(&text, &diagnostics_bag).map_err(|_|
            Self::create_compilation_unit(ast, diagnostics_bag)
        )?;
        let mut symbol_checker = SymbolChecker::new(Rc::clone(&diagnostics_bag));
        ast.visit(&mut symbol_checker);
        Self::check_diagnostics(&text, &diagnostics_bag)?;
    }

    fn create_compilation_unit(ast: Ast, diagnostics_bag: DiagnosticsBagCell) -> CompilationUnit {
        CompilationUnit {
            ast,
            diagnostics_bag,
        }
    }

    fn check_diagnostics(
        text: &text::SourceText,
        diagnostics_bag: &DiagnosticsBagCell
    ) -> Result<(), ()> {
        let diagnostics_binding = diagnostics_bag.borrow();

        if diagnostics_binding.diagnostics.len() > 0 {
            let diagnostics_printer = DiagnosticsPrinter::new(
                &text,
                &diagnostics_binding.diagnostics
            );
            diagnostics_printer.print();
            return Err(());
        }

        Ok(())
    }
}