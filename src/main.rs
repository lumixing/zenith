use std::{env, process, fs};

use scanner::Scanner;

mod token;
mod scanner;
mod ast;
mod parser;

// TODO: maybe way to make this static?
struct Zenith {
    had_error: bool
}

impl Zenith {
    pub fn error(&mut self, line: u32, message: &str) {
        println!("error at line {line}: {message}");
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("expected exactly 1 argument!");
        process::exit(256);
    }

    
    let path = &args[1];
    let source = fs::read_to_string(path).expect("that file does not exist!");
    let mut zenith = Zenith {
        had_error: false
    };
    let mut scanner = Scanner::from_source(source);
    let _tokens = scanner.scan_tokens(&mut zenith);
    scanner.print_tokens();

    if zenith.had_error {
        println!("exiting due to zenith error");
        process::exit(256);
    }
}
