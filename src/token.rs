#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Illegal, End,
    Identifier, Integer,
    Equals, Plus,
    Comma, Semicolor,
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Let, Function
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: Option<Literal>
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, literal: Option<Literal>) -> Self {
        Self {
            token_type,
            literal
        }
    }
}