use crate::{scanner::Scanner, token::Token};

struct Parser {
    scanner: Scanner,
    current: Option<Token>,
    peek: Option<Token>
}

impl Parser {
    fn new(scanner: Scanner) -> Self {
        let parser = Self {
            scanner,
            current: None,
            peek: None
        };

        parser
    }

    fn next_token(&mut self) {
        self.current = self.peek;
        self.peek = self.scanner.
    }
}