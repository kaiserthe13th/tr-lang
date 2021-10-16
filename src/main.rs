use std::fs;
use std::env;
use std::process::exit;

pub mod lexer;
use lexer::Lexer;

pub mod token;
pub mod util;

fn error_print<T>(error_name: &str, error_explanation: T)
where
    T: std::fmt::Debug
{
    eprintln!("{}: {:?}", error_name, error_explanation);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cont = String::new();
    if args.len() > 1 {
        let a = fs::read_to_string(args.get(1).unwrap());
        match a {
            Ok(s) => cont = s,
            Err(e) => error_print("error reading file", format!("{}", e)),
        }
    }
    let cont = cont;
    let mut lexer = Lexer::new(cont);
    let lexed = lexer.lex();
    if util::item_in_vec(&["--debug".to_string(), "-d".to_string()], &args) {
        println!("{:#?}", lexed);
    }
}
