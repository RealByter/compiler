use crate::lexer::{Keyword, Token};

#[derive(Debug)]

pub enum Expression {
    Constant(u32),
}

#[derive(Debug)]

pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]

pub enum FunctionDefinition {
    Function(String, Statement),
}

#[derive(Debug)]
pub enum Program {
    Program(FunctionDefinition),
}

pub fn parse_program(tokens: &mut Vec<Token>) -> Result<Program, String> {
    Ok(Program::Program(parse_function(tokens)?))
}

fn parse_function(tokens: &mut Vec<Token>) -> Result<FunctionDefinition, String> {
    expect(Token::Keyword(Keyword::Int), tokens)?;
    let identifier = parse_identifier(tokens)?;
    expect(Token::OpenParenthesis, tokens)?;
    expect(Token::Keyword(Keyword::Void), tokens)?;
    expect(Token::CloseParenthesis, tokens)?;
    expect(Token::OpenBrace, tokens)?;
    let statement = parse_statement(tokens)?;
    expect(Token::CloseBrace, tokens)?;

    Ok(FunctionDefinition::Function(identifier, statement))
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Result<String, String> {
    if tokens.is_empty() {
        return Err(String::from("Unexpected end of tokens."));
    }

    match tokens.remove(0) {
        Token::Identifier(identifier) => Ok(identifier),
        _ => Err(String::from("Invalid token. Expected an identifier")),
    }
}

fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, String> {
    expect(Token::Keyword(Keyword::Return), tokens)?;
    let expression = parse_expression(tokens)?;
    expect(Token::Semicolon, tokens)?;
    Ok(Statement::Return(expression))
}

fn parse_expression(tokens: &mut Vec<Token>) -> Result<Expression, String> {
    if tokens.is_empty() {
        return Err(String::from("Unexpected end of tokens."));
    }

    match tokens.remove(0) {
        Token::Constant(constant) => Ok(Expression::Constant(constant)),
        _ => Err(String::from("Invalid token. Expected an expression"))
    }
}

fn expect(expected: Token, tokens: &mut Vec<Token>) -> Result<(), String> {
    if tokens.is_empty() {
        return Err(String::from("Unexpected end of tokens."));
    }

    let token = &tokens[0];

    if token == &expected {
        tokens.remove(0);
        Ok(())
    } else {
        Err(format!(
            "Invalid token. Expected: {:?} got: {:?}",
            expected, token
        ))
    }
}
