use crate::ast::lexer::TextSpan;

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
        Diagnostic { message, span, kind };
    }
}

pub type DiagnosticsBagCell = Rc<RefCell<DiagnosticsBag>>;

pub struct DiagnosticsBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsBag {
    pub fn new() -> Self {
        DiagnosticsBag { diagnostics: vec![] };
    }

    pub fn report_error(&mut self, message: String, span: TextSpan) {
        let error = Diagnostic::new(message, span, DiagnosticKind::Error);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        let warning = Diagnostic::new(message, span, DiagnosticKind::Warning);
        self.diagnostics.push(warning);
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, token: &Token, span: TextSpan) {
        self.report_error(format!("Expected <{}>, Found <{}>", expected, token.kind), span);
    }
}
