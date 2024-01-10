use lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let input = "=;(),+{}\0".to_owned();
    let mut lexer = Lexer::new(input);
    while !lexer.is_at_end() {
        let token = lexer.next_token();
        println!("{:?}", token);
    }
}