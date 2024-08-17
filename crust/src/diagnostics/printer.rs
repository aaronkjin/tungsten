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
    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.get_line(line_index);
        let line_start = self.text.line_start(line_index);

        let column = diagnostic.span.start - line_start;
    }
}
