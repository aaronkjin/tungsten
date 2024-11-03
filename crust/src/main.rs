use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::ast::{
    Ast,
    ASTBinaryExpression,
    ASTLetStatement,
    ASTNumberExpression,
    ASTParenthesizedExpression,
    ASTVariableExpression,
    ASTVisitor,
};
use crate::ast::evaluator::ASTEvaluator;
use crate::ast::lexer::{ Lexer, TextSpan };
use crate::ast::parser::Parser;
use crate::compilation_unit::CompilationUnit;
use crate::diagnostics::DiagnosticsBagCell;
use crate::diagnostics::printer::DiagnosticsPrinter;
use crate::text::SourceText;

mod ast;
mod diagnostics;
mod text;
mod compilation_unit;

fn main() -> Result<(), ()> {
    // AST:
    /*
     * Variable decl
     *  Identifier: a
     *  Initializer: BinaryExpression
     *      Left: NumberExpression
     *          Number: 10
     *      Operator: Plus
     *      Right: NumberExpression
     *          Number: 30
     */

    // Flattened:
    /*
     * Decl, Binary Expr, Number, Number
     */

    let input =
        "\
        let a = 10 + 30
        let b = 20
        let d = 10 + e
        let c = (a + b) * d
    ";

    let compilation_unit = CompilationUnit::compile(input);
    compilation_unit.run();

    let Ok(());
}
