use crate::diagnostics::Diagnostic;

pub struct DiagnosticsPrinter<'a> {
    text: &'a SourceText,
    diagnostics: &'a [Diagnostic],
}
