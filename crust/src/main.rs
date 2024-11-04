use crate::compilation_unit::CompilationUnit;

mod ast;
mod diagnostics;
mod text;
mod compilation_unit;

fn main() {
    let input =
        "\
        let a = 10 + 30
        let b = 20
        let d = 10 + e
        let c = (a + b) * d
    ";

    let compilation_unit = CompilationUnit::compile(input);
    compilation_unit.run();
}
