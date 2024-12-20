use crate::lexer::{self, Keyword, Token};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub statement: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Unary(UnaryOperator, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Complement,
}

pub fn parse_program(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Program, String> {
    if tokens.peek().is_some() {
        Ok(Program {
            function: parse_function(tokens)?,
        })
    } else {
        Err("Unexpected end of tokens.".to_string())
    }
}

fn parse_function(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<FunctionDefinition, String> {
    expect(Token::Keyword(Keyword::Int), tokens)?;
    let identifier = parse_identifier(tokens)?;
    expect(Token::OpenParenthesis, tokens)?;
    expect(Token::Keyword(Keyword::Void), tokens)?;
    expect(Token::CloseParenthesis, tokens)?;
    expect(Token::OpenBrace, tokens)?;
    let statement = parse_statement(tokens)?;
    expect(Token::CloseBrace, tokens)?;

    Ok(FunctionDefinition {
        identifier,
        statement,
    })
}

fn parse_identifier(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<String, String> {
    match tokens.next() {
        Some(Token::Identifier(identifier)) => Ok(identifier),
        Some(token) => Err(format!(
            "Invalid token. Expected an identifier, got: {:?}",
            token
        )),
        None => Err("Unexpected end of tokens.".to_string()),
    }
}

fn parse_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    expect(Token::Keyword(Keyword::Return), tokens)?;
    let expression = parse_expression(tokens)?;
    expect(Token::Semicolon, tokens)?;
    Ok(Statement::Return(expression))
}

fn parse_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    match tokens.next() {
        Some(Token::Constant(constant)) => Ok(Expression::Constant(constant)),
        Some(Token::UnaryOperator(operator)) => {
            let expression = parse_expression(tokens)?;
            Ok(Expression::Unary(
                parse_unary_operator(operator)?,
                Box::new(expression),
            ))
        }
        Some(Token::OpenParenthesis) => {
            let expression = parse_expression(tokens)?;
            expect(Token::CloseParenthesis, tokens)?;
            Ok(expression)
        }
        Some(token) => Err(format!(
            "Invalid token. Expected a constant, got: {:?}",
            token
        )),
        None => Err("Unexpected end of tokens.".to_string()),
    }
}

fn parse_unary_operator(op: lexer::UnaryOperator) -> Result<UnaryOperator, String> {
    match op {
        lexer::UnaryOperator::Negate => Ok(UnaryOperator::Negate),
        lexer::UnaryOperator::Complement => Ok(UnaryOperator::Complement),
        _ => return Err(format!("Unsupported unary operator: {:?}", op)),
    }
}

fn expect(
    expected: Token,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<(), String> {
    match tokens.next() {
        Some(token) => {
            if token == expected {
                Ok(())
            } else {
                Err(format!(
                    "Invalid token. Expected: {:?} got: {:?}",
                    expected, token
                ))
            }
        }
        None => Err(format!(
            "Unexpected end of tokens. Expected: {:?}",
            expected
        )),
    }
}
