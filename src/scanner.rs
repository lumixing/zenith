use std::collections::HashMap;

use substring::Substring;

use crate::{token::{Token, TokenType, Literal}, Zenith};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32
}

impl Scanner {
    pub fn from_source(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{token}");
        }
    }

    pub fn scan_tokens(&mut self, zenith: &mut Zenith) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(zenith);
        }

        self.tokens.push(Token {
            token_type: TokenType::End,
            lexeme: "".to_owned(),
            literal: None,
            line: self.line,
        });

        &self.tokens
    }

    fn scan_token(&mut self, zenith: &mut Zenith) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen,  None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace,  None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma,      None),
            '.' => self.add_token(TokenType::Dot,        None),
            '-' => self.add_token(TokenType::Minus,      None),
            '+' => self.add_token(TokenType::Plus,       None),
            ';' => self.add_token(TokenType::Semicolon,  None),
            '*' => self.add_token(TokenType::Star,       None),
            // TODO: extract matches outside so its cleaner,
            // too scared to do it right now ;-;
            '!' => {
                let matches = self.matches('=');
                self.add_token(
                    if matches { TokenType::BangEqual }
                          else { TokenType::Bang }, 
                    None)
            },
            '=' => {
                let matches = self.matches('=');
                self.add_token(
                    if matches { TokenType::EqualEqual }
                          else { TokenType::Equal }, 
                    None)
            },
            '<' => {
                let matches = self.matches('=');
                self.add_token(
                    if matches { TokenType::LessEqual }
                          else { TokenType::Less }, 
                    None)
            },
            '>' => {
                let matches = self.matches('=');
                self.add_token(
                    if matches { TokenType::GreaterEqual }
                          else { TokenType::Greater }, 
                    None)
            },
            '/' => {
                let matches = self.matches('/');
                if matches {
                    // comment lasts until end of line or file
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            },
            ' ' | '\r' | '\t' => { /* ignore whitespace */ },
            '\n' => self.line += 1, 
            '"' => self.string(zenith),
            c => {
                if Scanner::is_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    zenith.error(self.line, "unexpected character!")
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn get_char(&self, i: usize) -> char {
        self.source.chars().nth(i)
            .expect("tried to get char in source out of range!")
    }

    fn current_char(&self) -> char {
        self.get_char(self.current as usize)
    }

    /// returns current char and increases current by 1
    fn advance(&mut self) -> char {
        let current_char = self.current_char();
        self.current += 1;
        current_char
    }

    /// pushes new token from source substring (from start to current)
    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source.substring(self.start as usize, self.current as usize);
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_owned(),
            literal,
            line: self.line,
        })
    }

    /// returns true and increases current by 1 if current char is expected char,
    /// otherwise just return false
    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.current_char() != expected { return false }
        self.current += 1;
        true
    }

    /// returns current character
    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.current_char()
    }

    fn string(&mut self, zenith: &mut Zenith) {
        // string lasts until closing quote or end of file (multi-line string)
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            zenith.error(self.line, "unterminated string!");
            return;
        }

        // closing "
        self.advance();

        // trim surrounding quotes
        let value = self.source.substring(self.start as usize + 1, self.current as usize - 1);
        self.add_token(TokenType::String, Some(Literal::String(value.to_owned())));
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        // look for fractional part
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // consume "."
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let str_value = self.source.substring(self.start as usize, self.current as usize);
        let value: f32 = str_value.parse()
            .expect("could not parse float!");
        self.add_token(TokenType::Number, Some(Literal::Float(value)))
    }

    // return next char
    fn peek_next(&self) -> char {
        if self.current as usize + 1 >= self.source.len() { return '\0' }
        self.get_char(self.current as usize + 1)
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
         c == '_'
    }
    
    fn is_alphanumeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let keywords = Scanner::keywords_map();
        let text = self.source.substring(self.start as usize, self.current as usize);
        let token_type = keywords.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token(*token_type, None);
    }

    fn keywords_map() -> HashMap<&'static str, TokenType> {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map
    }
}
