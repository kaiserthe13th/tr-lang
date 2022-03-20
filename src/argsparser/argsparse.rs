#[cfg(feature = "interactive")]
use crate::interactive::QuietLevel;
#[cfg(feature = "fmt")]
use crate::fmt::{LineEnding, IndentOptions};
use crate::store::globarg::*;
use crate::util;
use crate::utilbin;
use std::env;

#[derive(Debug)]
pub enum Subcommands {
    Run,
    Byt,
    RunBytes,
    Command,
    #[cfg(feature = "interactive")]
    Interact,
    #[cfg(feature = "fmt")]
    Format,
}

#[derive(Debug)]
pub struct Options {
    pub name: String,
    pub file: String,
    pub outfile: Option<String>,
    pub prd_out: bool,
    pub sub_cmd: Subcommands,
    pub help: bool,
    pub help_exitc: i32, // I am too lazy to remove this from everywhere
    pub version: bool,
    pub lex_out: bool,
    pub prs_out: bool,
    pub argv: Vec<String>,
    pub license: bool,
    #[cfg(feature = "interactive")]
    pub quiet: QuietLevel,
    #[cfg(feature = "fmt")]
    pub line_ending: LineEnding,
    #[cfg(feature = "fmt")]
    pub indent: IndentOptions,
}

pub fn parse_args() -> Options {
    let mut args: Vec<String> = env::args().collect();

    let name = args.get(0).unwrap().to_string();
    args.remove(0);

    let mut argv_m = false;
    let mut argv: Vec<String> = vec![];

    #[cfg(feature = "interactive")]
    let mut quiet = QuietLevel::None;
    #[cfg(all(feature = "fmt", not(windows)))]
    let mut line_ending = LineEnding::LF;
    #[cfg(all(feature = "fmt", windows))]
    let mut line_ending = LineEnding::CRLF;
    #[cfg(feature = "fmt")]
    let mut indent = IndentOptions::Spaces(4);

    let (mut help, mut version) = (false, false);

    let (mut lex_out, mut prs_out) = (false, false);
    let (mut prd_out, mut outfile) = (false, None);

    if args.len() == 1 {
        match args[0].as_str() {
            "-V" | "-s" | "--sürüm" => {
                utilbin::print_version(name);
            }
            "-h" | "-y" | "--yardım" => utilbin::print_help(0, name),
            "-L" | "--license" | "--lisans" => utilbin::print_license(),
            "i" | "inter" => (),
            _ => utilbin::print_help(1, name),
        }
        if args.len() == 1 && args[0] != "inter" && args[0] != "i" && args[0] != "fmt" && args[0] != "f" {
            utilbin::print_help(1, name);
        }
    }

    let sub_cmd = if args.len() >= 1 {
        let s = match args.get(0).unwrap().as_str() {
            "y" | "yürüt" => Subcommands::Run,
            "b" | "byt" => Subcommands::Byt,
            "yb" | "yürbyt" => Subcommands::RunBytes,
            "k" | "komut" => Subcommands::Command,
            #[cfg(feature = "interactive")]
            "i" | "inter" => Subcommands::Interact,
            #[cfg(feature = "fmt")]
            "f" | "fmt" => Subcommands::Format,
            "-h" | "-y" | "--yardım" => {
                utilbin::print_help(0, name);
            }
            "-V" | "-s" | "--sürüm" => {
                utilbin::print_version(name);
            }
            "-L" | "--license" | "--lisans" => {
                utilbin::print_license();
            }
            a => util::error_print("unknown subcommand", format!("{}", a)),
        };
        args.remove(0);
        s
    } else {
        #[cfg(feature = "interactive")]
        { Subcommands::Interact }
        #[cfg(not(feature = "interactive"))]
        utilbin::print_help(1, name)
    };

    let mut outs = false;
    let mut license = false;

    #[cfg(feature = "interactive")]
    let file = if let Subcommands::Interact = sub_cmd {
        "".to_string()
    } else {
        let file = args.get(0).expect("couldn't get <FILE>").to_string();
        args.remove(0);
        file
    };
    #[cfg(not(feature = "interactive"))]
    let file = {
        let file = args.get(0).expect("couldn't get <FILE>").to_string();
        args.remove(0);
        file
    };

    #[cfg(feature = "fmt")]
    let mut change_line_ending = false;
    #[cfg(feature = "fmt")]
    let mut change_indent = false;
    for arg in args {
        match arg.as_str() {
            a if argv_m => argv.push(a.to_string()),
            a if outs => {
                outs = false;
                outfile.replace(a.to_string());
            }
            #[cfg(feature = "fmt")]
            a if change_line_ending => {
                match a {
                    "lf" => line_ending = LineEnding::LF,
                    "crlf" => line_ending = LineEnding::CRLF,
                    _ => utilbin::print_help(1, name),
                }
            }
            #[cfg(feature = "fmt")]
            a if change_indent => {
                indent = if a.chars().all(|a| a.is_ascii_digit()) {
                    let spc = a.parse().unwrap();
                    IndentOptions::Spaces(spc)
                } else {
                    match a {
                        "t" | "tabs" => IndentOptions::Tabs,
                        _ => utilbin::print_help(1, name),
                    }
                }
            }
            "-h" | "-y" | "--yardım" => help = true,
            "-V" | "-s" | "--sürüm" => version = true,
            "-l" | "--lexer-çıktısı" => lex_out = true,
            "-u" | "--uyarıları-engelle" => unsafe {
                SUPRESS_WARN = true;
            },
            "-p" | "--parser-çıktısı" => prs_out = true,
            "-o" | "-ç" | "--çıkış" => {
                outs = true;
                prd_out = true;
            }
            "-L" | "--license" | "--lisans" => license = true,
            #[cfg(feature = "interactive")]
            "-q" | "--sessiz" => quiet.inc(),
            #[cfg(feature = "interactive")]
            "-qq" => quiet.inc_by(2),
            #[cfg(feature = "interactive")]
            "-qqq" => quiet.inc_by(3),
            "--" => argv_m = true,
            #[cfg(feature = "fmt")]
            "--lending" | "--satson" => {
                change_line_ending = true;
            },
            #[cfg(feature = "fmt")]
            "-i" | "--indent" | "--girinti" => {
                change_indent = true;
            }
            a => util::error_print("unknown argument", format!("{}", a)),
        }
    }

    Options {
        name,
        help,
        version,
        argv,
        lex_out,
        prs_out,
        prd_out,
        outfile,
        file,
        sub_cmd,
        license,
        help_exitc: 0,
        #[cfg(feature = "interactive")]
        quiet,
        #[cfg(feature = "fmt")]
        line_ending,
        #[cfg(feature = "fmt")]
        indent,
    }
}
