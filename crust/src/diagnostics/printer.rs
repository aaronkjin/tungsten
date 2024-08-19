use crate::diagnostics::Diagnostic;

pub struct DiagnosticsPrinter<'a> {
    text: &'a SourceText,
    diagnostics: &'a [Diagnostic],
}

const PREFIX_LENGTH: usize = 8;

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
        let prefix_start = cmp::max(0, (column as isize) - (PREFIX_LENGTH as isize)) as usize;
        let prefix_end = column;
        let suffix_start = cmp::min(column + diagnostic.span.length(), line.len()) + 1;
        let suffix_end = cmp::min(suffix_start + PREFIX_LENGTH, line.len());

        let prefix = &line[prefix_start..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];

        let indent = cmp::max(cmp::min(PREFIX_LENGTH, column), 0) as usize;
        // let indent = cmp::min(PREFIX_LENGTH, column);
        let arrow_pointers = format!("{:indent$}^", "", indent = indent);
        let arrow_line = format!("{:indent$}|", "", indent = indent);
    }
}
