use crate::ast::ASTVisitor;

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.value);
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        expr.left.visit(self);
        let left = self.last_value.unwrap();

        expr.right.visit(self);
        let right = self.last_value.unwrap();

        self.last_value = Some(match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => left + right,
            ASTBinaryOperatorKind::Minus => left - right,
        });
    }
}
