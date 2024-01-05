use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // punctuation
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Plus, Minus, Semicolon, Slash, Star,

    // one or two char
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier, String, Number,

    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    End
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Float(f32)
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}