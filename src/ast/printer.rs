use termion::color;
use termion::color::{ Fg, Reset };

use crate::ast::{
    ASTAssignmentExpression,
    ASTBinaryExpression,
    ASTBlockStatement,
    ASTBooleanExpression,
    ASTCallExpression,
    ASTFuncDeclStatement,
    ASTIfStatement,
    ASTLetStatement,
    ASTNumberExpression,
    ASTParenthesizedExpression,
    ASTReturnStatement,
    ASTStatement,
    ASTUnaryExpression,
    ASTVariableExpression,
    ASTWhileStatement,
};
use crate::ast::lexer::TextSpan;
use crate::ast::visitor::ASTVisitor;

pub struct ASTPrinter {
    pub indent: usize,
    pub result: String,
}

impl ASTPrinter {
    const NUMBER_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Magenta = color::Magenta;
    const VARIABLE_COLOR: color::Green = color::Green;
    const BOOLEAN_COLOR: color::Yellow = color::Yellow;

    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_newline(&mut self) {
        self.result.push_str("\n");
    }

    pub fn new() -> Self {
        Self { indent: 0, result: String::new() }
    }
}

impl<'a> ASTVisitor<'a> for ASTPrinter {
    fn visit_func_decl_statement(&mut self, func_decl_statement: &ASTFuncDeclStatement) {
        self.result.push_str(&format!("{}func", Self::KEYWORD_COLOR.fg_str()));
        self.add_whitespace();
        self.result.push_str(
            &format!(
                "{}{}",
                Self::VARIABLE_COLOR.fg_str(),
                func_decl_statement.identifier.span.literal
            )
        );
        self.result.push_str(&format!("{}(", Self::TEXT_COLOR.fg_str()));

        for (i, param) in func_decl_statement.parameters.iter().enumerate() {
            if i > 0 {
                self.result.push_str(&format!("{}, ", Self::TEXT_COLOR.fg_str()));
            }
            self.result.push_str(
                &format!("{}{}", Self::VARIABLE_COLOR.fg_str(), param.identifier.span.literal)
            );
        }

        self.result.push_str(&format!("{})", Self::TEXT_COLOR.fg_str()));
        self.visit_statement(&func_decl_statement.body);
    }

    fn visit_return_statement(&mut self, return_statement: &ASTReturnStatement) {
        self.result.push_str(&format!("{}return", Self::KEYWORD_COLOR.fg_str()));

        if let Some(return_value) = &return_statement.return_value {
            self.add_whitespace();
            self.visit_expression(return_value);
        }
    }

    fn visit_while_statement(&mut self, while_statement: &ASTWhileStatement) {
        self.result.push_str(&format!("{}while", Self::KEYWORD_COLOR.fg_str()));
        self.add_whitespace();
        self.visit_expression(&while_statement.condition);
        self.visit_statement(&while_statement.body);
    }

    fn visit_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        self.result.push_str(&format!("{} {{", Self::TEXT_COLOR.fg_str()));
        self.add_newline();
        self.indent += 1;

        for statement in &block_statement.statements {
            for _ in 0..self.indent {
                self.result.push_str("    ");
            }
            self.visit_statement(statement);
            self.add_newline();
        }

        self.indent -= 1;
        for _ in 0..self.indent {
            self.result.push_str("    ");
        }
        self.result.push_str(&format!("{}}}", Self::TEXT_COLOR.fg_str()));
    }

    fn visit_if_statement(&mut self, if_statement: &ASTIfStatement) {
        self.result.push_str(&format!("{}if", Self::KEYWORD_COLOR.fg_str()));
        self.add_whitespace();
        self.visit_expression(&if_statement.condition);
        self.visit_statement(&if_statement.then_branch);

        if let Some(else_branch) = &if_statement.else_branch {
            self.add_whitespace();
            self.result.push_str(&format!("{}else", Self::KEYWORD_COLOR.fg_str()));
            self.visit_statement(&else_branch.else_statement);
        }
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.result.push_str(&format!("{}let", Self::KEYWORD_COLOR.fg_str()));
        self.add_whitespace();
        self.result.push_str(
            &format!("{}{}", Self::VARIABLE_COLOR.fg_str(), let_statement.identifier.span.literal)
        );
        self.add_whitespace();
        self.result.push_str(&format!("{}=", Self::TEXT_COLOR.fg_str()));
        self.add_whitespace();
        self.visit_expression(&let_statement.initializer);
    }

    fn visit_assignment_expression(&mut self, assignment_expression: &ASTAssignmentExpression) {
        self.result.push_str(
            &format!(
                "{}{}",
                Self::VARIABLE_COLOR.fg_str(),
                assignment_expression.identifier.span.literal
            )
        );
        self.add_whitespace();
        self.result.push_str(&format!("{}=", Self::TEXT_COLOR.fg_str()));
        self.add_whitespace();
        self.visit_expression(&assignment_expression.expression);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        self.result.push_str(
            &format!(
                "{}{}",
                Self::VARIABLE_COLOR.fg_str(),
                variable_expression.identifier.span.literal
            )
        );
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.result.push_str(&format!("{}{}", Self::NUMBER_COLOR.fg_str(), number.number));
    }

    fn visit_boolean_expression(&mut self, boolean: &ASTBooleanExpression) {
        self.result.push_str(&format!("{}{}", Self::BOOLEAN_COLOR.fg_str(), boolean.value));
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal));
    }

    fn visit_unary_expression(&mut self, unary_expression: &ASTUnaryExpression) {
        self.result.push_str(
            &format!(
                "{}{}",
                Self::TEXT_COLOR.fg_str(),
                unary_expression.operator.token.span.literal
            )
        );
        self.visit_expression(&unary_expression.operand);
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        // Left
        self.visit_expression(&binary_expression.left);
        self.add_whitespace();

        // Operator
        self.result.push_str(
            &format!(
                "{}{}",
                Self::TEXT_COLOR.fg_str(),
                binary_expression.operator.token.span.literal
            )
        );

        // Right
        self.add_whitespace();
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    ) {
        // Left
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), "("));
        self.visit_expression(&parenthesized_expression.expression);
        // Right
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), ")"));
    }

    fn visit_call_expression(&mut self, call_expression: &ASTCallExpression) {
        self.result.push_str(
            &format!("{}{}", Self::VARIABLE_COLOR.fg_str(), call_expression.identifier.span.literal)
        );
        self.result.push_str(&format!("{}(", Self::TEXT_COLOR.fg_str()));

        for (i, arg) in call_expression.arguments.iter().enumerate() {
            if i > 0 {
                self.result.push_str(&format!("{}, ", Self::TEXT_COLOR.fg_str()));
            }
            self.visit_expression(arg);
        }

        self.result.push_str(&format!("{})", Self::TEXT_COLOR.fg_str()));
    }
}
