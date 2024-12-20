use lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Void,
    Int,
    Return,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Constant(u32),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Keyword(k1 ), Token::Keyword(k2)) => k1 == k2,
            (Token::Identifier(_), Token::Identifier(_)) => true,        
            (Token::Constant(_), Token::Constant(_)) => true,
            (Token::OpenParenthesis, Token::OpenParenthesis) => true,        
            (Token::CloseParenthesis, Token::CloseParenthesis) => true,      
            (Token::OpenBrace, Token::OpenBrace) => true,                    
            (Token::CloseBrace, Token::CloseBrace) => true,                  
            (Token::Semicolon, Token::Semicolon) => true,                    
            _ => false,
        }
    } 
}

struct TokenPattern {
    regex: Regex,
    token_type: fn(&str) -> Token,
}

lazy_static::lazy_static! {
    static ref TOKEN_PATTERNS: Vec<TokenPattern> = vec![
        TokenPattern {
            regex: Regex::new(r"\bint\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Int),
        },
        TokenPattern {
            regex: Regex::new(r"\breturn\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Return),
        },
        TokenPattern{
            regex: Regex::new(r"\bvoid\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Void),
        },
        TokenPattern {
            regex: Regex::new(r"[a-zA-Z_]\w*\b").unwrap(),
            token_type: |lexeme| Token::Identifier(lexeme.to_string()),
        },
        TokenPattern {
            regex: Regex::new(r"[0-9]+\b").unwrap(),
            token_type: |lexeme| Token::Constant(lexeme.parse().unwrap()),
        },
        TokenPattern {
            regex: Regex::new(r"\(").unwrap(),
            token_type: |_| Token::OpenParenthesis,
        },
        TokenPattern {
            regex: Regex::new(r"\)").unwrap(),
            token_type: |_| Token::CloseParenthesis,
        },
        TokenPattern {
            regex: Regex::new(r"\{").unwrap(),
            token_type: |_| Token::OpenBrace,
        },
        TokenPattern {
            regex: Regex::new(r"\}").unwrap(),
            token_type: |_| Token::CloseBrace,
        },
        TokenPattern {
            regex: Regex::new(r";").unwrap(),
            token_type: |_| Token::Semicolon,
        },
    ];
}

fn match_token(input: &str) -> Option<(Token, usize)> {
    let mut longest_match: Option<(Token, usize)> = None;

    for pattern in TOKEN_PATTERNS.iter() {
        if let Some(mat) = pattern.regex.find(input) {
            if mat.start() == 0 {
                let length = mat.len();
                let token = (pattern.token_type)(&input[..length]);
                match &longest_match {
                    Some((_, longest_length)) => {
                        if length > *longest_length {
                            longest_match = Some((token, length))
                        }
                    },
                    None => {
                        longest_match = Some((token, length))
                    }
                }
            }
        }
    }

    longest_match
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Some((token, length)) = match_token(remaining) {
            tokens.push(token);
            remaining = (&remaining[length..]).trim_start();
        } else {
            panic!("Unexpected token at: {}", remaining);
        }
    }

    tokens
}
