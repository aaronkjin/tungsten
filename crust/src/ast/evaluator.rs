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
    fn visit_let_statement(&mut self, _let_statement: &ASTLetStatement) {
        todo!()
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

    fn visit_parenthesized_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.visit_expression(&expr.expression);
    }

    fn visit_variable_expression(&mut self, _expr: &ASTVariableExpression) {
        // TODO: Implement variable lookup logic
        self.last_value = None;
    }
}
