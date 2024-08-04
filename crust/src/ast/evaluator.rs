use crate::ast::ASTVisitor;

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.value);
    }
}
