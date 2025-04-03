use crate::compilation_unit::CompilationUnit;

mod ast;
mod diagnostics;
mod text;
mod compilation_unit;

// Precedence: paren, unary, mult/div, add/sub, bitwise (shift, AND, XOR, OR)
fn main() {
    let input = "\
        let b = 1
        let a = (1 + 2) * -------b & 3
    ";

    let compilation_unit = CompilationUnit::compile(input);
    compilation_unit.maybe_run();
}
