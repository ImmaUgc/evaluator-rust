use super::Visitor;


pub struct Evaluator {
    pub last_value: Option<i64>
}

impl Evaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl Visitor for Evaluator {
    fn visit_number(&mut self, number: &super::NumberExpression) {
        self.last_value = Some(number.number)
    }

    fn visit_binary_expression(&mut self, binary_expr: &super::BinaryExpression) {
        self.visit_expression(&binary_expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&binary_expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match binary_expr.operator.kind {
            super::Operator::Plus => left + right,
            super::Operator::Minus => left - right,
            super::Operator::Multiply => left * right,
            super::Operator::Divide => left / right,
        })
    }
}