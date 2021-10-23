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

pub mod runtime;

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
    if args.help == true {
        util::print_help(args.help_exitc, args.name);
    }
    if args.version {
        util::print_version(args.name);
    }
    
    
    match args.sub_cmd {
        argsparser::Subcommands::Byt => {
            let mut lexer = Lexer::new({
                let mut my_file = fs::File::open(&args.file).unwrap();

                let mut buf = String::new();
                my_file.read_to_string(&mut buf).unwrap();
                buf
            });
            let lexed = lexer.clone().lex();
            if args.lex_out {
                println!("{:#?}", &lexed);
            }
        
            let mut parser = Parser::from_lexer(&mut lexer);
            let parsed = parser.parse();
            if args.prs_out {
                println!("{:#?}", parsed.clone());
            }

            let encoded = bytecode::to_bytecode(parsed.clone());
            {
                let mut bytecode_src = fs::File::create(&match &args.outfile {
                    Some(f) => f.clone(),
                    None => format!("{}.trbyt", args.file),
                }).unwrap();
                
                bytecode_src.write_all(&encoded[..]).unwrap();
            }
        },
        argsparser::Subcommands::Run => {
            // TODO: if specified accept args.outfile
            let mut lexer = Lexer::new({
                let mut my_file = match fs::File::open(&args.file) {
                    Err(e) => error_print("error opening file", format!("{}: {}", e, &args.file)),
                    Ok(f) => f,
                };

                let mut buf = String::new();
                my_file.read_to_string(&mut buf).unwrap();
                buf
            });
            let lexed = lexer.clone().lex();
            if args.lex_out {
                println!("{:#?}", lexed);
            }
        
            let mut parser = Parser::from_lexer(&mut lexer);
            let parsed = parser.clone().parse();
            if args.prs_out {
                println!("{:#?}", parsed);
            }

            let mut run = runtime::Run::new(parser.parse());
            run.run();
        },
        argsparser::Subcommands::RunBytes => {
            let mut bytecode_src = match fs::File::open(&args.file) {
                Err(e) => error_print("could not open file", format!("{}: {}", e, args.file)),
                Ok(f) => f,
            };

            let mut con: Vec<u8> = vec![];
            bytecode_src.read_to_end(&mut con).unwrap();

            let parsed = bytecode::from_bytecode(&con[..]);

            let mut run = runtime::Run::new(parsed);
            run.run();
        },
    }
}
