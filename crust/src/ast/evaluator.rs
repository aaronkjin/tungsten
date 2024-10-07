use crate::ast::{
    ASTBinaryExpression,
    ASTBinaryOperatorKind,
    ASTLetStatement,
    ASTNumberExpression,
    ASTVisitor,
    TextSpan,
    ASTParenthesizedExpression,
    ASTVariableExpression,
};

use std::collections::HashMap;

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
    pub variables: HashMap<String, i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self { last_value: None, variables: HashMap::new() }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);

        // Only insert if evaluation is successful
        if let Some(value) = self.last_value {
            self.variables.insert(let_statement.identifier.span.literal.clone(), value);
        } else {
            // Handle case where the expression resulted in an error
            println!(
                "Error: Could not evaluate let statement for variable {}",
                let_statement.identifier.span.literal
            );
        }
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_error(&mut self, _span: &TextSpan) {
        println!("Error: Invalid expression or token found.");
        self.last_value = None;
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = match self.last_value {
            Some(value) => value,
            None => {
                println!("Error: Could not evaluate left side of binary expression.");
                return;
            }
        };

        self.visit_expression(&expr.right);
        let right = match self.last_value {
            Some(value) => value,
            None => {
                println!("Error: Could not evaluate right side of binary expression.");
                return;
            }
        };

        self.last_value = Some(match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        });
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        let var_name = &variable_expression.identifier.span.literal;

        if let Some(value) = self.variables.get(var_name) {
            self.last_value = Some(*value);
        } else {
            // Handle the case where the variable is undeclared
            println!("Error: Undeclared variable '{}'", var_name);
            self.last_value = None;
        }
    }
}

/*
impl ASTVisitor for ASTEvaluator {
    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
        self.variables.insert(
            let_statement.identifier.span.literal.clone(),
            self.last_value.unwrap()
        );
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_error(&mut self, _span: &TextSpan) {
        // FIXME: Implement error handling logic here
        self.last_value = None;
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();

        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();

        self.last_value = Some(match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        });
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        self.last_value = Some(
            *self.variables.get(&variable_expression.identifier.span.literal).unwrap()
        );
    }
}
*/
