#![feature(io_error_more)]

use std::fs;
use std::io::Write;

pub use std::process::exit;
use std::path::PathBuf;

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
            let mut path = PathBuf::from(&args.file);
            let mut lexer = Lexer::new(match util::read_file(&path) {
                Ok(f) => f,
                Err(util::FSErr::IsADir) => {
                    path.push("main.trl");
                    util::read_file(&path).unwrap()
                },
            });
            if args.lex_out {
                let lexed = lexer.clone().tokenize(&mut vec![args.file.clone()], args.file.clone()); 
                println!("{:#?}", &lexed);
            }
        
            let mut parser = Parser::from_lexer(&mut lexer, args.file.clone());
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
            let mut path = PathBuf::from(args.file.clone());
            let mut lexer = Lexer::new(match util::read_file(&path) {
                Ok(f) => f,
                Err(util::FSErr::IsADir) => {
                    path.push("main.trl");
                    util::read_file(&path).unwrap()
                },
            });
            if args.lex_out {
                let lexed = lexer.clone().tokenize(&mut vec![args.file.clone()], args.file.clone());
                println!("{:#?}", lexed);
            }
        
            let mut parser = Parser::from_lexer(&mut lexer, args.file);
            if args.prs_out {
                let parsed = parser.clone().parse();
                println!("{:#?}", parsed);
            }

            let mut run = runtime::Run::new(parser.parse());
            run.run();
        },
        argsparser::Subcommands::RunBytes => {
            let path = PathBuf::from(args.file);
            let con = util::read_file_to_vec_u8(&path);
            let parsed = bytecode::from_bytecode(&con[..]);

            let mut run = runtime::Run::new(parsed);
            run.run();
        },
    }
}
