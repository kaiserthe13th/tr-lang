#![feature(io_error_more)]

use std::fs;
use std::fs::canonicalize;
use std::io::Write;

use std::path::PathBuf;
use std::process::exit;

pub mod lexer;
use lexer::Lexer;

pub mod parser;
use parser::Parser;

pub mod bytecode;
pub mod mem;
pub mod store;
pub mod token;
mod util;
mod utilbin;

pub mod errwarn;
pub mod runtime;

mod argsparser;
#[cfg(feature = "interactive")]
mod interactive;

fn main() {
    let args = argsparser::parse_args();
    if args.help == true {
        utilbin::print_help(args.help_exitc, args.name);
    }
    if args.version {
        utilbin::print_version(args.name);
    }
    if args.license {
        utilbin::print_license();
    }

    match args.sub_cmd {
        argsparser::Subcommands::Byt => {
            let mut path = PathBuf::from(&args.file);
            let mut lexer = Lexer::new(match util::read_file(&path) {
                Ok(f) => f,
                Err(util::FSErr::IsADir) => {
                    path.push("main.trl");
                    util::read_file(&path).unwrap()
                }
            });
            if args.lex_out {
                let canon_path = match canonicalize(args.file.clone()) {
                    Ok(a) => a.as_path().display().to_string(),
                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", args.file.clone(), e),
                };
                let lexed = lexer
                    .clone()
                    .tokenize(&mut vec![canon_path.clone()], canon_path);
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
                })
                .unwrap();

                bytecode_src.write_all(&encoded[..]).unwrap();
            }
        }
        argsparser::Subcommands::Run => {
            // TODO: if specified accept args.outfile
            let mut path = PathBuf::from(args.file.clone());
            let mut lexer = Lexer::new(match util::read_file(&path) {
                Ok(f) => f,
                Err(util::FSErr::IsADir) => {
                    path.push("main.trl");
                    util::read_file(&path).unwrap()
                }
            });
            if args.lex_out {
                let canon_path = match canonicalize(args.file.clone()) {
                    Ok(a) => a.as_path().display().to_string(),
                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", args.file.clone(), e),
                };
                let lexed = lexer
                    .clone()
                    .tokenize(&mut vec![canon_path.clone()], canon_path);
                println!("{:#?}", lexed);
            }

            let mut parser = Parser::from_lexer(&mut lexer, args.file.clone());
            if args.prs_out {
                let parsed = parser.clone().parse();
                println!("{:#?}", parsed);
            }

            let mut run = runtime::Run::new(parser.parse());
            run.run(args.file, None, false).unwrap_or_else(|(_, _, a)| a.error());
        }
        argsparser::Subcommands::RunBytes => {
            let path = PathBuf::from(args.file.clone());
            let con = utilbin::read_file_to_vec_u8(&path);
            let parsed = bytecode::from_bytecode(&con[..]);

            let mut run = runtime::Run::new(parsed);
            run.run(args.file, None, false).unwrap_or_else(|(_, _, a)| a.error());
        }
        argsparser::Subcommands::Command => {
            runtime::Run::new(
                Parser::from_lexer(
                    &mut Lexer::new(args.file),
                    "<args>".to_string()
                ).parse()
            ).run("<args>".to_string(), None, false).unwrap_or_else(|(_, _, a)| a.error());
        }
        #[cfg(feature = "interactive")]
        argsparser::Subcommands::Interact => {
            interactive::Interactive::new(args.quiet).start()
        }
    }
}
