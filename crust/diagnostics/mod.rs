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

pub struct DiagnosticsBag {
    pub diagnostics: Vec<Diagnostic>,
}
