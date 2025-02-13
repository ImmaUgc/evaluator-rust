use super::{lexer::{ Token, TokenKind }, ASTExpression, BinaryOperator, Operator, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().filter(|t| t.kind != TokenKind::Whitespace).map(|t| t.clone()).collect(),
            current: 0
        }
    }

    pub fn next_statement(&mut self) -> Option<Statement> {
        let token = self.current()?;
        if token.kind == TokenKind::Eof {
            return None;
        }

        let expr = self.parse_expression()?;
        Some(Statement::expression(expr))

    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_operator() {
            self.consume();
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?;
            left = ASTExpression::binary(operator, left, right);
        }

        Some(left)
    }

    fn parse_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.current()?;
        let kind = match token.kind {
            TokenKind::Plus => Some(Operator::Plus),
            TokenKind::Minus => Some(Operator::Minus),
            TokenKind::Asterisk => Some(Operator::Multiply),
            TokenKind::Slash => Some(Operator::Divide),
            _ => None
        };
        kind.map(|kind| BinaryOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;
        match token.kind {
            TokenKind::Number(number) => Some(ASTExpression::number(number)),
            TokenKind::LParen => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != TokenKind::RParen {
                    panic!("Missing parenthesis");
                }
                Some(ASTExpression::parenthesis(expr))
            }
            _ => None
        }
    }

    fn peek(&mut self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&mut self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1);
        token
    }
}