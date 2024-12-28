use lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Void,
    Int,
    Return,
    If,
    Else,
    Do,
    While,
    For,
    Break,
    Continue,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Constant(i64),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Minus,
    Complement,
    Decrement,
    Plus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    Not,
    LAnd,
    LOr,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessOrEqual,
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
    TernaryIf,
    TernaryElse
    // Discarded because lookahead is not supported - will be added when I make my own state machine to parse tokens
    // PrefixIncrement,
    // PrefixDecrement,
    // PostfixIncrement,
    // PostfixDecrement,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Keyword(k1), Token::Keyword(k2)) => k1 == k2,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Constant(_), Token::Constant(_)) => true,
            (Token::OpenParenthesis, Token::OpenParenthesis) => true,
            (Token::CloseParenthesis, Token::CloseParenthesis) => true,
            (Token::OpenBrace, Token::OpenBrace) => true,
            (Token::CloseBrace, Token::CloseBrace) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::Operator(o1), Token::Operator(o2)) => o1 == o2,
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
            regex: Regex::new(r"\bif\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::If),
        },
        TokenPattern {
            regex: Regex::new(r"\belse\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Else),
        },
        TokenPattern {
            regex: Regex::new(r"\bdo\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Do),
        },
        TokenPattern {
            regex: Regex::new(r"\bwhile\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::While),
        },
        TokenPattern {
            regex: Regex::new(r"\bfor\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::For),
        },
        TokenPattern {
            regex: Regex::new(r"\bbreak\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Break),
        },
        TokenPattern {
            regex: Regex::new(r"\bcontinue\b").unwrap(),
            token_type: |_| Token::Keyword(Keyword::Continue),
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
        TokenPattern {
            regex: Regex::new(r"-").unwrap(),
            token_type: |_| Token::Operator(Operator::Minus),
        },
        TokenPattern {
            regex: Regex::new(r"~").unwrap(),
            token_type: |_| Token::Operator(Operator::Complement),
        },
        TokenPattern {
            regex: Regex::new(r"--").unwrap(),
            token_type: |_| Token::Operator(Operator::Decrement),
        },
        TokenPattern {
            regex: Regex::new(r"\+").unwrap(),
            token_type: |_| Token::Operator(Operator::Plus),
        },
        TokenPattern {
            regex: Regex::new(r"\*").unwrap(),
            token_type: |_| Token::Operator(Operator::Multiply),
        },
        TokenPattern {
            regex: Regex::new(r"/").unwrap(),
            token_type: |_| Token::Operator(Operator::Divide),
        },
        TokenPattern {
            regex: Regex::new(r"%").unwrap(),
            token_type: |_| Token::Operator(Operator::Modulo),
        },
        TokenPattern {
            regex: Regex::new(r"&").unwrap(),
            token_type: |_| Token::Operator(Operator::And),
        },
        TokenPattern {
            regex: Regex::new(r"\|").unwrap(),
            token_type: |_| Token::Operator(Operator::Or),
        },
        TokenPattern {
            regex: Regex::new(r"\^").unwrap(),
            token_type: |_| Token::Operator(Operator::Xor),
        },
        TokenPattern {
            regex: Regex::new(r"<<").unwrap(),
            token_type: |_| Token::Operator(Operator::ShiftLeft),
        },
        TokenPattern {
            regex: Regex::new(r">>").unwrap(),
            token_type: |_| Token::Operator(Operator::ShiftRight),
        },
        TokenPattern {
            regex: Regex::new(r"!").unwrap(),
            token_type: |_| Token::Operator(Operator::Not)
        },
        TokenPattern {
            regex: Regex::new(r"&&").unwrap(),
            token_type: |_| Token::Operator(Operator::LAnd),
        },
        TokenPattern {
            regex: Regex::new(r"\|\|").unwrap(),
            token_type: |_| Token::Operator(Operator::LOr),
        },
        TokenPattern {
            regex: Regex::new(r"==").unwrap(),
            token_type: |_| Token::Operator(Operator::EqualTo),
        },
        TokenPattern {
            regex: Regex::new(r"!=").unwrap(),
            token_type: |_| Token::Operator(Operator::NotEqualTo),
        },
        TokenPattern {
            regex: Regex::new(r"<").unwrap(),
            token_type: |_| Token::Operator(Operator::LessThan),
        },
        TokenPattern {
            regex: Regex::new(r">").unwrap(),
            token_type: |_| Token::Operator(Operator::GreaterThan),
        },
        TokenPattern {
            regex: Regex::new(r"<=").unwrap(),
            token_type: |_| Token::Operator(Operator::LessOrEqual),
        },
        TokenPattern {
            regex: Regex::new(r">=").unwrap(),
            token_type: |_| Token::Operator(Operator::GreaterOrEqual),
        },
        TokenPattern {
            regex: Regex::new(r"=").unwrap(),
            token_type: |_| Token::Operator(Operator::Assign),
        },
        TokenPattern {
            regex: Regex::new(r"\+=").unwrap(),
            token_type: |_| Token::Operator(Operator::AddAssign),
        },
        TokenPattern {
            regex: Regex::new(r"-=").unwrap(),
            token_type: |_| Token::Operator(Operator::SubAssign),
        },
        TokenPattern {
            regex: Regex::new(r"\*=").unwrap(),
            token_type: |_| Token::Operator(Operator::MulAssign),
        },
        TokenPattern {
            regex: Regex::new(r"/=").unwrap(),
            token_type: |_| Token::Operator(Operator::DivAssign),
        },
        TokenPattern {
            regex: Regex::new(r"%=").unwrap(),
            token_type: |_| Token::Operator(Operator::ModAssign),
        },
        TokenPattern {
            regex: Regex::new(r"&=").unwrap(),
            token_type: |_| Token::Operator(Operator::AndAssign),
        },
        TokenPattern {
            regex: Regex::new(r"\|=").unwrap(),
            token_type: |_| Token::Operator(Operator::OrAssign),
        },
        TokenPattern {
            regex: Regex::new(r"\^=").unwrap(),
            token_type: |_| Token::Operator(Operator::XorAssign),
        },
        TokenPattern {
            regex: Regex::new(r"<<=").unwrap(),
            token_type: |_| Token::Operator(Operator::LeftShiftAssign),
        },
        TokenPattern {
            regex: Regex::new(r">>=").unwrap(),
            token_type: |_| Token::Operator(Operator::RightShiftAssign),
        },
        // TokenPattern {
        //     regex: Regex::new(r"(?<!\w)\+\+").unwrap(),
        //     token_type: |_| Token::Operator(Operator::PrefixIncrement),
        // },
        // TokenPattern {
        //     regex: Regex::new(r"(?<!\w)--").unwrap(),
        //     token_type: |_| Token::Operator(Operator::PrefixDecrement),
        // },
        // TokenPattern {
        //     regex: Regex::new(r"(?<=\w)\+\+(?!\w)").unwrap(),
        //     token_type: |_| Token::Operator(Operator::PostfixIncrement),
        // },
        // TokenPattern {
        //     regex: Regex::new(r"(?<=\w)--(?!\w)").unwrap(),
        //     token_type: |_| Token::Operator(Operator::PostfixDecrement),
        // },
        TokenPattern {
            regex: Regex::new(r"\?").unwrap(),
            token_type: |_| Token::Operator(Operator::TernaryIf),
        },
        TokenPattern {
            regex: Regex::new(r":").unwrap(),
            token_type: |_| Token::Operator(Operator::TernaryElse),
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
                    }
                    None => longest_match = Some((token, length)),
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
