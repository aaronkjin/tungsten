use crate::ast::{
    ASTAssignmentExpression,
    ASTBinaryExpression,
    ASTBlockStatement,
    ASTBooleanExpression,
    ASTCallExpression,
    ASTExpression,
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

pub trait ASTVisitor<'a> {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            crate::ast::ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
            crate::ast::ASTStatementKind::Let(expr) => {
                self.visit_let_statement(expr);
            }
            crate::ast::ASTStatementKind::If(expr) => {
                self.visit_if_statement(expr);
            }
            crate::ast::ASTStatementKind::Block(expr) => {
                self.visit_block_statement(expr);
            }
            crate::ast::ASTStatementKind::While(expr) => {
                self.visit_while_statement(expr);
            }
            crate::ast::ASTStatementKind::FuncDecl(expr) => {
                self.visit_func_decl_statement(expr);
            }
            crate::ast::ASTStatementKind::Return(expr) => {
                self.visit_return_statement(expr);
            }
        }
    }

    fn visit_return_statement(&mut self, statement: &ASTReturnStatement);

    fn visit_func_decl_statement(&mut self, statement: &ASTFuncDeclStatement);

    fn visit_while_statement(&mut self, statement: &ASTWhileStatement);

    fn visit_block_statement(&mut self, statement: &ASTBlockStatement);

    fn visit_if_statement(&mut self, statement: &ASTIfStatement);

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement);

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            crate::ast::ASTExpressionKind::Number(number) => {
                self.visit_number_expression(number);
            }
            crate::ast::ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            crate::ast::ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            crate::ast::ASTExpressionKind::Error(span) => {
                self.visit_error(span);
            }
            crate::ast::ASTExpressionKind::Variable(expr) => {
                self.visit_variable_expression(expr);
            }
            crate::ast::ASTExpressionKind::Unary(expr) => {
                self.visit_unary_expression(expr);
            }
            crate::ast::ASTExpressionKind::Assignment(expr) => {
                self.visit_assignment_expression(expr);
            }
            crate::ast::ASTExpressionKind::Boolean(expr) => {
                self.visit_boolean_expression(expr);
            }
            crate::ast::ASTExpressionKind::Call(expr) => {
                self.visit_call_expression(expr);
            }
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_call_expression(&mut self, call_expression: &ASTCallExpression);

    fn visit_boolean_expression(&mut self, boolean_expression: &ASTBooleanExpression);

    fn visit_assignment_expression(&mut self, assignment_expression: &ASTAssignmentExpression);

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression);

    fn visit_number_expression(&mut self, number: &ASTNumberExpression);

    fn visit_error(&mut self, span: &TextSpan);

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression);

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    );

    fn visit_unary_expression(&mut self, unary_expression: &ASTUnaryExpression);
}
