use lexer::Token;

pub mod lexer;
pub mod parser;
pub mod evaluator;

#[derive(Debug)]

pub enum StatementKind {
    Expression(ASTExpression)
}

#[derive(Debug)]
pub struct Ast {
    statements: Vec<Statement>
}

impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn Visitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }
}

pub trait Visitor {
    fn visit_statement(&mut self, statement: &Statement) {
        match &statement.kind {
            StatementKind::Expression(expr) => self.visit_expression(expr)
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ExpressionKind::Number(number) => self.visit_number(number),
            ExpressionKind::Binary(binary_expression) => self.visit_binary_expression(binary_expression),
            ExpressionKind::Parenthesis(parenthesis_expression) => self.visit_parenthesis_expression(parenthesis_expression),

        }
    }

    fn visit_number(&mut self, number: &NumberExpression);

    fn visit_binary_expression(&mut self, binary_expr: &BinaryExpression) {
        self.visit_expression(&binary_expr.left);
        self.visit_expression(&binary_expr.right);
    }

    fn visit_parenthesis_expression(&mut self, expr: &ParenthesisExpression) {
        self.visit_expression(&expr.expression);
    }
}

#[derive(Debug)]

pub struct Statement {
    kind: StatementKind
}

impl Statement {
    pub fn new(kind: StatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        Self { kind: StatementKind::Expression(expr) }
    }
}

#[derive(Debug)]

pub enum ExpressionKind {
    Number(NumberExpression),
    Binary(BinaryExpression),
    Parenthesis(ParenthesisExpression)
}

#[derive(Debug)]

struct ASTExpression {
    kind: ExpressionKind
}

impl ASTExpression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }

    pub fn number(number: i64) -> Self {
        Self::new(ExpressionKind::Number(NumberExpression { number }))
    }

    pub fn binary(operator: BinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        Self::new(ExpressionKind::Binary(BinaryExpression { left: Box::new(left), operator, right: Box::new(right) }))
    }

    pub fn parenthesis(expression: ASTExpression) -> Self {
        Self::new(ExpressionKind::Parenthesis(ParenthesisExpression { expression: Box::new(expression) }))
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    left: Box<ASTExpression>,
    operator: BinaryOperator,
    right: Box<ASTExpression>,
}

#[derive(Debug)]

pub struct NumberExpression {
    number: i64
}

#[derive(Debug)]

pub struct  ParenthesisExpression {
    expression: Box<ASTExpression>
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide
}

#[derive(Debug)]
pub struct BinaryOperator {
    kind: Operator,
    token: Token
}

impl BinaryOperator {
    pub fn new(kind: Operator, token: Token) -> Self {
        Self { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            Operator::Plus => 1,
            Operator::Minus => 1,
            Operator::Multiply => 2,
            Operator::Divide => 2
        }
    }
}