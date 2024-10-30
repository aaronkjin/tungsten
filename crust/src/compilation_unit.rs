use crate::ast::Ast;
use crate::diagnostics::DiagnosticsBagCell;

pub struct CompilationUnit {
    ast: Ast,
    diagnostics_bag: DiagnosticsBagCell,
}

impl CompilationUnit {
    pub fn compile(input: &str) -> CompilationUnit {}
}
