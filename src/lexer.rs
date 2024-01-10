use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: i32,
    character: char,
    read_position: i32
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            character: '\0',
            read_position: 0,
        };

        lexer.read_character();
        lexer
    }

    fn read_character(&mut self) {
        if self.is_at_end() {
            self.character = '\0';
        } else {
            self.character = self.input.chars().nth(self.read_position as usize).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len() as i32
    }

    pub fn next_token(&mut self) -> Token {
        // if self.is_at_end() {
        //     return Token::new(TokenType::End, None);
        // }

        self.skip_whitespace();

        let token = match self.character {
            '=' => Token::new(TokenType::Equals, None),
            ';' => Token::new(TokenType::Semicolor, None),
            '(' => Token::new(TokenType::LeftParen, None),
            ')' => Token::new(TokenType::RightParen, None),
            ',' => Token::new(TokenType::Comma, None),
            '+' => Token::new(TokenType::Plus, None),
            '{' => Token::new(TokenType::LeftBrace, None),
            '}' => Token::new(TokenType::RightBrace, None),
            '\0' => Token::new(TokenType::End, None),
            _ => Token::new(TokenType::Illegal, None)
        };

        self.read_character();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.character == ' '
            || self.character == '\t'
            || self.character == '\n'
            || self.character == '\r' {
            self.read_character()
        }
    }
}