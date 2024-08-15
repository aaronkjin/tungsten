use crate::diagnostics::Diagnostic;

pub struct DiagnosticsPrinter<'a> {
    text: &'a SourceText,
    diagnostics: &'a [Diagnostic],
}

impl<'a> DiagnosticsPrinter<'a> {
    pub fn new(text: &'a SourceText, diagnostics: &'a [Diagnostic]) -> Self {
        Self {
            text,
            diagnostics,
        }
    }

    /*
     * Stringify the diagnostic by using the format:
     * let <red>x<reset> = 5;
     *          ^
     *          |
     *          +-- This is the error message (<line>:<column>)
     */
    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> Result<String, ()> {}
}
