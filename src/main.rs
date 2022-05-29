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
use util::{get_lang, SupportedLanguage};
mod utilbin;
#[cfg(feature = "fmt")]
pub mod fmt;

pub mod error;
use error::Error;
pub mod runtime;

pub mod ffi;

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
                    path.push("giriş.trl");
                    match util::read_file(&path) {
                        Ok(f) => f,
                        Err(e) => match get_lang() {    
                            SupportedLanguage::Turkish => Error::new(
                                "DosyaHatası",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None
                            ),
                            SupportedLanguage::English => Error::new(
                                "FSError",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None,
                            ),
                        }.error(),
                    }
                }
                Err(e) => match get_lang() {    
                    SupportedLanguage::Turkish => Error::new(
                        "DosyaHatası",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None
                    ),
                    SupportedLanguage::English => Error::new(
                        "FSError",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None,
                    ),
                }.error(),
            });
            if args.lex_out {
                let canon_path = match canonicalize(path) {
                    Ok(a) => a.display().to_string(),
                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", args.file.clone(), e),
                };
                let lexed = lexer
                    .clone()
                    .tokenize(&mut vec![canon_path.clone()], canon_path)
                    .unwrap_or_else(|e| e.error());
                println!("{:#?}", &lexed);
            }

            let mut parser = match Parser::from_lexer(&mut lexer, args.file.clone()) {
                Ok(p) => p,
                Err(e) => e.error(),
            };
            let parsed = parser.parse().unwrap_or_else(|e| e.error());
            if args.prs_out {
                println!("{:#?}", parsed.clone());
            }

            let encoded = bytecode::to_bytecode(parsed.clone());
            {
                let mut bytecode_src = fs::File::create(&match &args.outfile {
                    Some(f) => f.clone(),
                    None => {
                        let mut pb = PathBuf::from(args.file);
                        pb.set_extension("trbyt");
                        pb.display().to_string()
                    }
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
                    path.push("giriş.trl");
                    match util::read_file(&path) {
                        Ok(f) => f,
                        Err(e) => match get_lang() {    
                            SupportedLanguage::Turkish => Error::new(
                                "DosyaHatası",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None
                            ),
                            SupportedLanguage::English => Error::new(
                                "FSError",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None,
                            ),
                        }.error(),
                    }
                }
                Err(e) => match get_lang() {    
                    SupportedLanguage::Turkish => Error::new(
                        "DosyaHatası",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None
                    ),
                    SupportedLanguage::English => Error::new(
                        "FSError",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None,
                    ),
                }.error(),
            });
            if args.lex_out {
                let canon_path = match canonicalize(path) {
                    Ok(a) => a.as_path().display().to_string(),
                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", args.file.clone(), e),
                };
                let lexed = lexer
                    .clone()
                    .tokenize(&mut vec![canon_path.clone()], canon_path)
                    .unwrap_or_else(|e| e.error());
                println!("{:#?}", lexed);
            }

            let parser = match Parser::from_lexer(&mut lexer, args.file.clone()) {
                Ok(p) => p,
                Err(e) => e.error(),
            };
            let parsed = parser.clone().parse().unwrap_or_else(|e| e.error());
            if args.prs_out {
                println!("{:#?}", &parsed);
            }

            let mut run = runtime::Run::new(parsed);
            run.run(runtime::RunConfig {
                file: args.file,
                supress_warnings: args.supress_warnings,
                ..Default::default()
            })
                .unwrap_or_else(|(s, h, a)| { a.auto(); (s, h) });
        }
        argsparser::Subcommands::RunBytes => {
            let path = PathBuf::from(args.file.clone());
            let con = utilbin::read_file_to_vec_u8(&path);
            let parsed = bytecode::from_bytecode(&con[..]);

            let mut run = runtime::Run::new(parsed);
            run.run(runtime::RunConfig {
                file: args.file,
                supress_warnings: args.supress_warnings,
                ..Default::default()
            })
                .unwrap_or_else(|(s, h, a)| { a.auto(); (s, h) });
        }
        argsparser::Subcommands::Command => {
            runtime::Run::new(
                match Parser::from_lexer(&mut Lexer::new(args.file), ".".to_string()) {
                    Ok(p) => p,
                    Err(e) => e.error(),
                }.parse().unwrap_or_else(|e| e.error()),
            )
            .run(runtime::RunConfig {
                supress_warnings: args.supress_warnings,
                ..Default::default()
            })
            .unwrap_or_else(|(s, h, a)| { a.auto(); (s, h) });
        }
        #[cfg(feature = "interactive")]
        argsparser::Subcommands::Interact => interactive::Interactive::new(args.quiet, interactive::InteractiveOptions::default()).start(),
        #[cfg(feature = "fmt")]
        argsparser::Subcommands::Format => {
            // TODO: if specified accept args.outfile
            let mut path = PathBuf::from(args.file.clone());
            let mut lexer = Lexer::new(match util::read_file(&path) {
                Ok(f) => f,
                Err(util::FSErr::IsADir) => {
                    path.push("giriş.trl");
                    match util::read_file(&path) {
                        Ok(f) => f,
                        Err(e) => match get_lang() {    
                            SupportedLanguage::Turkish => Error::new(
                                "DosyaHatası",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None
                            ),
                            SupportedLanguage::English => Error::new(
                                "FSError",
                                &format!("{:?}", e),
                                vec![(0, 0, path.display().to_string(), None)],
                                None,
                            ),
                        }.error(),
                    }
                }
                Err(e) => match get_lang() {    
                    SupportedLanguage::Turkish => Error::new(
                        "DosyaHatası",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None
                    ),
                    SupportedLanguage::English => Error::new(
                        "FSError",
                        &format!("{:?}", e),
                        vec![(0, 0, path.display().to_string(), None)],
                        None,
                    ),
                }.error(),
            });
            let canon_path = match canonicalize(path) {
                Ok(a) => a.as_path().display().to_string(),
                Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", args.file.clone(), e),
            };
            let lexed = lexer
                .do_post_proc(false)
                .tokenize(&mut vec![canon_path.clone()], canon_path)
                .unwrap_or_else(|e| e.error());
            if args.lex_out {
                println!("{:#?}", &lexed);
            }

            let fmtr = fmt::Fmt::new(lexed)
                .line_ending(args.line_ending)
                .indent(args.indent);
            let res = fmtr.fmt();
            println!("{res}");
        }
    }
}
