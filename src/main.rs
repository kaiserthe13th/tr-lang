use std::fs;
use std::io::Write;
use std::io::Read;

pub use std::process::exit;

pub mod lexer;
use lexer::Lexer;

pub mod parser;
use parser::Parser;

pub mod token;
pub mod store;
pub mod util;
pub mod bytecode;

mod argsparser;

pub fn error_print<T>(error_name: &str, error_explanation: T) -> !
where
    T: std::fmt::Debug
{
    eprintln!("{}: {:?}", error_name, error_explanation);
    exit(1);
}

fn main() {
    let args = argsparser::parse_args();
    
    let mut lexer = Lexer::new("cont".to_string());
    let lexed = lexer.lex();
    if args.lex_out {
        println!("{:?}", &lexed);
    }

    let mut parser = Parser::from_lexer(&mut lexer);
    let parsed = parser.parse();
    if args.prs_out {
        println!("{:?}", &lexed);
    }

    match args.sub_cmd {
        argsparser::Subcommands::Byt => {
            let encoded = bytecode::to_bytecode(parsed);
            {
                let mut bytecode_src = fs::File::create(&match args.outfile {
                    Some(f) => f,
                    None => format!("{}.trbyt", args.file),
                }).unwrap();
        
                bytecode_src.write_all(&encoded[..]).unwrap();
            }
        },
        argsparser::Subcommands::Run => {
            // Run when runtime implemented
        },
        argsparser::Subcommands::RunBytes => {
            let mut bytecode_src = match fs::File::open(&args.file) {
                Err(e) => error_print("could not open file", format!("{}: {}", e, args.file)),
                Ok(f) => f,
            };

            let mut con: Vec<u8> = vec![];
            bytecode_src.read_to_end(&mut con).unwrap();

            // let parsed = bytecode::from_bytecode(&con[..]);

            // Run when runtime implemented
        },
    }
    
}
