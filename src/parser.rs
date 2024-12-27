use lazy_static::lazy_static;

use crate::lexer::{self, Keyword, Token};
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Program {
    pub function: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    S(Statement),
    D(Declaration),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    Null,
}

#[derive(Debug)]
pub enum Declaration {
    Initialized(String, Expression),
    Uninitialized(String),
}

#[derive(Debug)]
pub enum Expression {
    Var(String),
    Constant(i64),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Option<BinaryOperator>, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Complement,
    Not,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Xor,
    And,
    Or,
    LeftShift,
    RightShift,
    LAnd,
    LOr,
    EqualTo,
    NotEqualTo,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

const MAX_PRECEDENCE: u8 = 150;
lazy_static! {
    static ref PRECEDENCE_MAP: HashMap<BinaryOperator, u8> = {
        let mut map = HashMap::new();
        map.insert(BinaryOperator::Multiply, 30);
        map.insert(BinaryOperator::Divide, 30);
        map.insert(BinaryOperator::Modulo, 30);
        map.insert(BinaryOperator::Add, 40);
        map.insert(BinaryOperator::Subtract, 40);
        map.insert(BinaryOperator::LeftShift, 50);
        map.insert(BinaryOperator::RightShift, 50);
        map.insert(BinaryOperator::LessThan, 60);
        map.insert(BinaryOperator::LessOrEqual, 60);
        map.insert(BinaryOperator::GreaterThan, 60);
        map.insert(BinaryOperator::GreaterOrEqual, 60);
        map.insert(BinaryOperator::EqualTo, 70);
        map.insert(BinaryOperator::NotEqualTo, 70);
        map.insert(BinaryOperator::And, 80);
        map.insert(BinaryOperator::Xor, 90);
        map.insert(BinaryOperator::Or, 100);
        map.insert(BinaryOperator::LAnd, 110);
        map.insert(BinaryOperator::LOr, 120);
        map.insert(BinaryOperator::Assign, 140);
        map.insert(BinaryOperator::AddAssign, 140);
        map.insert(BinaryOperator::SubAssign, 140);
        map.insert(BinaryOperator::MulAssign, 140);
        map.insert(BinaryOperator::DivAssign, 140);
        map.insert(BinaryOperator::ModAssign, 140);
        map.insert(BinaryOperator::AndAssign, 140);
        map.insert(BinaryOperator::OrAssign, 140);
        map.insert(BinaryOperator::XorAssign, 140);
        map.insert(BinaryOperator::LeftShiftAssign, 140);
        map.insert(BinaryOperator::RightShiftAssign, 140);
        map
    };
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
    let mut body: Vec<BlockItem> = Vec::new();
    while match tokens.peek() {
        Some(Token::CloseBrace) | None => false,
        _ => true,
    } {
        body.push(parse_block_item(tokens)?);
    }
    expect(Token::CloseBrace, tokens)?;

    Ok(FunctionDefinition { identifier, body })
}

fn parse_block_item(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<BlockItem, String> {
    if let Some(Token::Keyword(Keyword::Int)) = tokens.peek() {
        Ok(BlockItem::D(parse_declaration(tokens)?))
    } else {
        Ok(BlockItem::S(parse_statement(tokens)?))
    }
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

fn parse_declaration(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Declaration, String> {
    expect(Token::Keyword(Keyword::Int), tokens)?;
    let identifier = parse_identifier(tokens)?;
    if let Some(Token::Operator(lexer::Operator::Assign)) = tokens.peek() {
        tokens.next();
        let expression = parse_expression(tokens, MAX_PRECEDENCE)?;
        expect(Token::Semicolon, tokens)?;
        Ok(Declaration::Initialized(identifier, expression))
    } else {
        expect(Token::Semicolon, tokens)?;
        Ok(Declaration::Uninitialized(identifier))
    }
}

fn parse_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    match tokens.peek() {
        Some(Token::Keyword(Keyword::Return)) => {
            tokens.next();
            let expression = parse_expression(tokens, MAX_PRECEDENCE)?;
            expect(Token::Semicolon, tokens)?;
            Ok(Statement::Return(expression))
        }
        Some(Token::Semicolon) => {
            tokens.next();
            Ok(Statement::Null)
        }
        Some(_) => {
            let expression = parse_expression(tokens, MAX_PRECEDENCE)?;
            expect(Token::Semicolon, tokens)?;
            Ok(Statement::Expression(expression))
        }
        None => return Err("Unexpected end of tokens.".to_string()),
    }
}

fn parse_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    max_precedence: u8,
) -> Result<Expression, String> {
    let mut left = parse_factor(tokens)?;
    while let Some(Token::Operator(
        op @ (lexer::Operator::Plus
        | lexer::Operator::Minus
        | lexer::Operator::Multiply
        | lexer::Operator::Divide
        | lexer::Operator::Modulo
        | lexer::Operator::And
        | lexer::Operator::Or
        | lexer::Operator::Xor
        | lexer::Operator::ShiftLeft
        | lexer::Operator::ShiftRight
        | lexer::Operator::LAnd
        | lexer::Operator::LOr
        | lexer::Operator::EqualTo
        | lexer::Operator::NotEqualTo
        | lexer::Operator::LessThan
        | lexer::Operator::LessOrEqual
        | lexer::Operator::GreaterThan
        | lexer::Operator::GreaterOrEqual
        | lexer::Operator::Assign
        | lexer::Operator::AddAssign
        | lexer::Operator::SubAssign
        | lexer::Operator::MulAssign
        | lexer::Operator::DivAssign
        | lexer::Operator::ModAssign
        | lexer::Operator::AndAssign
        | lexer::Operator::OrAssign
        | lexer::Operator::XorAssign
        | lexer::Operator::LeftShiftAssign
        | lexer::Operator::RightShiftAssign),
    )) = tokens.peek()
    {
        let op = parse_binary_operator(&op)?;
        let precedence = *PRECEDENCE_MAP.get(&op).unwrap();
        if precedence >= max_precedence {
            break;
        }
        match op {
            // Right to left associativity
            BinaryOperator::Assign => {
                tokens.next();
                let right = parse_expression(tokens, precedence)?;
                left = Expression::Assignment(None, Box::new(left), Box::new(right));
            }
            BinaryOperator::AddAssign
            | BinaryOperator::SubAssign
            | BinaryOperator::MulAssign
            | BinaryOperator::DivAssign
            | BinaryOperator::ModAssign
            | BinaryOperator::AndAssign
            | BinaryOperator::OrAssign
            | BinaryOperator::XorAssign
            | BinaryOperator::LeftShiftAssign
            | BinaryOperator::RightShiftAssign => {
                let op = match op {
                    BinaryOperator::AddAssign => BinaryOperator::Add,
                    BinaryOperator::SubAssign => BinaryOperator::Subtract,
                    BinaryOperator::MulAssign => BinaryOperator::Multiply,
                    BinaryOperator::DivAssign => BinaryOperator::Divide,
                    BinaryOperator::ModAssign => BinaryOperator::Modulo,
                    BinaryOperator::AndAssign => BinaryOperator::And,
                    BinaryOperator::OrAssign => BinaryOperator::Or,
                    BinaryOperator::XorAssign => BinaryOperator::Xor,
                    BinaryOperator::LeftShiftAssign => BinaryOperator::LeftShift,
                    BinaryOperator::RightShiftAssign => BinaryOperator::RightShift,
                    _ => return Err("Shouldn't reach here".to_string()),
                };
                tokens.next();
                let right = parse_expression(tokens, precedence)?;
                left = Expression::Assignment(Some(op), Box::new(left), Box::new(right));
            }
            // Left to right associativity
            _ => {
                tokens.next();
                let right: Expression = parse_expression(tokens, precedence - 1)?;
                left = Expression::Binary(op, Box::new(left), Box::new(right));
            }
        }
    }
    Ok(left)
}

fn parse_factor(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, String> {
    match tokens.next() {
        Some(Token::Constant(constant)) => Ok(Expression::Constant(constant)),
        Some(Token::Operator(
            operator
            @ (lexer::Operator::Minus | lexer::Operator::Complement | lexer::Operator::Not),
        )) => {
            // let expression = parse_expression(tokens)?;
            let inner_expression = parse_factor(tokens)?;
            Ok(Expression::Unary(
                parse_unary_operator(operator)?,
                Box::new(inner_expression),
            ))
        }
        Some(Token::OpenParenthesis) => {
            let inner_expression = parse_expression(tokens, MAX_PRECEDENCE)?;
            expect(Token::CloseParenthesis, tokens)?;
            Ok(inner_expression)
        }
        Some(Token::Identifier(id)) => Ok(Expression::Var(id)),
        Some(token) => {
            print!("Remaining: ");
            for token in tokens {
                print!("{:?}", token);
            }
            println!();
            Err(format!(
                "Invalid token. Expected a factor, got: {:?}",
                token
            ))
        }
        None => Err("Unexpected end of tokens.".to_string()),
    }
}

fn parse_unary_operator(op: lexer::Operator) -> Result<UnaryOperator, String> {
    match op {
        lexer::Operator::Minus => Ok(UnaryOperator::Negate),
        lexer::Operator::Complement => Ok(UnaryOperator::Complement),
        lexer::Operator::Not => Ok(UnaryOperator::Not),
        _ => return Err(format!("Unsupported unary operator: {:?}", op)),
    }
}

fn parse_binary_operator(op: &lexer::Operator) -> Result<BinaryOperator, String> {
    match op {
        lexer::Operator::Plus => Ok(BinaryOperator::Add),
        lexer::Operator::Minus => Ok(BinaryOperator::Subtract),
        lexer::Operator::Multiply => Ok(BinaryOperator::Multiply),
        lexer::Operator::Divide => Ok(BinaryOperator::Divide),
        lexer::Operator::Modulo => Ok(BinaryOperator::Modulo),
        lexer::Operator::Xor => Ok(BinaryOperator::Xor),
        lexer::Operator::And => Ok(BinaryOperator::And),
        lexer::Operator::Or => Ok(BinaryOperator::Or),
        lexer::Operator::ShiftLeft => Ok(BinaryOperator::LeftShift),
        lexer::Operator::ShiftRight => Ok(BinaryOperator::RightShift),
        lexer::Operator::LAnd => Ok(BinaryOperator::LAnd),
        lexer::Operator::LOr => Ok(BinaryOperator::LOr),
        lexer::Operator::EqualTo => Ok(BinaryOperator::EqualTo),
        lexer::Operator::NotEqualTo => Ok(BinaryOperator::NotEqualTo),
        lexer::Operator::LessThan => Ok(BinaryOperator::LessThan),
        lexer::Operator::LessOrEqual => Ok(BinaryOperator::LessOrEqual),
        lexer::Operator::GreaterThan => Ok(BinaryOperator::GreaterThan),
        lexer::Operator::GreaterOrEqual => Ok(BinaryOperator::GreaterOrEqual),
        lexer::Operator::Assign => Ok(BinaryOperator::Assign),
        _ => return Err(format!("Unsupported binary operator: {:?}", op)),
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
                print!("Remaining: ");
                for token in tokens {
                    print!("{:?},", token);
                }
                println!();
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
