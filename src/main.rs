mod ast;

use ast::{lexer::Lexer, parser::Parser, Ast, evaluator::Evaluator};

fn main() {
    let mut lex = Lexer::new("(907 + 92) + 1");
    let mut tokens = Vec::new();

    while let Some(token) = lex.next_token() {
        tokens.push(token);
    }

    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    // println!("{:?}", ast);

    let mut eval = Evaluator::new();

    ast.visit(&mut eval);

    println!("{}", eval.last_value.unwrap());
}