use std::fs;
use std::env;
pub use std::process::exit;

pub mod lexer;
use lexer::Lexer;

pub mod parser;
use parser::Parser;

pub mod token;
pub mod store;
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
    if util::item_in_vec(&["--help".to_string(), "-h".to_string()], &args) {
        util::print_help(0, args[0].clone());
    }
    if util::item_in_vec(&["--version".to_string(), "-v".to_string()], &args) {
        util::print_version(args[0].clone());
    }
    let mut cont = String::new();
    if args.len() > 1 {
        let a = fs::read_to_string(args.get(1).unwrap());
        match a {
            Ok(s) => cont = s,
            Err(e) => error_print("error reading file", format!("{}", e)),
        }
    } else {
        util::print_help(1, args[0].clone());
    }
    let cont = cont;
    let mut lexer = Lexer::new(cont);
    let lexed = lexer.clone().lex();
    if util::item_in_vec(&["--show-lex-result".to_string(), "-l".to_string()], &args) {
        println!("{:#?}", lexed);
    }
    let mut parser = Parser::from_lexer(&mut lexer);
    let parsed = parser/* .clone() */.parse(); // reserved for future when our parsed token vector are put into bytecode::creator
    if util::item_in_vec(&["--show-parse-result".to_string(), "-p".to_string()], &args) {
        println!("{:#?}", parsed);
    }
}
