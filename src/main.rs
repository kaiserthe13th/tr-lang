use std::fs;
use std::env;

pub mod lexer;
use lexer::Lexer;

pub mod token;
pub mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cont = String::new();
    if args.len() > 1 {
        cont = fs::read_to_string(args.get(1).unwrap()).unwrap();
    }
    let cont = cont;
    let mut lexer = Lexer::new(cont);
    let lexed = lexer.lex();
    println!("{:#?}", lexed);
}
