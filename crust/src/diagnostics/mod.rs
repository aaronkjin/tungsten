pub mod printer;

use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::lexer::{ TextSpan, Token, TokenKind };

#[derive(Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Clone)]
pub struct Diagnostic {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
        Diagnostic { message, span, kind }
    }
}

pub type DiagnosticsBagCell = Rc<RefCell<DiagnosticsBag>>;

pub struct DiagnosticsBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsBag {
    pub fn new() -> Self {
        DiagnosticsBag { diagnostics: vec![] }
    }

    pub fn report_error(&mut self, message: String, span: TextSpan) {
        let error = Diagnostic::new(message, span, DiagnosticKind::Error);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        let warning = Diagnostic::new(message, span, DiagnosticKind::Warning);
        self.diagnostics.push(warning);
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, token: &Token) {
        self.report_error(
            format!("Expected <{}>, found <{}>", expected, token.kind),
            token.span.clone()
        );
    }
    pub fn report_expected_expression(&mut self, token: &Token) {
        self.report_error(
            format!("Expected expression, found <{}>", token.kind),
            token.span.clone()
        );
    }

    pub fn report_undeclared_variable(&mut self, token: &Token) {
        self.report_error(
            format!("Undeclared variable '{}'", token.span.literal),
            token.span.clone()
        );
    }
}

#[cfg(test)]
mod test {
    use crate::diagnostics::Diagnostic;

    struct DiagnosticsVerifier {
        actual: Vec<Diagnostic>,
        expected: Vec<Diagnostic>,
    }

    impl DiagnosticsVerifier {
        pub fn new(input: &str, messages: Vec<&str>) -> Self {
            let expected = Self::parse_input(input, messages);
            let actual = Self::compile(input);
            Self { expected, actual }
        }

        fn compile(input: &str) -> Vec<Diagnostic> {
            let compilation_unit = CompilationUnit::compile(input);
            let diagnostics = compilation_unit.diagnostics_bag.borrow();
            diagnostics.diagnostics.clone()
        }

        fn parse_input(input: &str, messages: Vec<&str>) -> Vec<Diagnostic> {}

        fn verify(&self) {}
    }

    #[test]
    fn should_report_undeclared_variable() {
        let input = "let a = «b»";
        let expected = vec!["Undeclared variable 'b'"];

        let verifier = DiagnosticsVerifier::new(input, expected);
    }
}
