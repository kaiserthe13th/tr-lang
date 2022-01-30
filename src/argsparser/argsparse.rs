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
}

pub fn parse_args() -> Options {
    let mut args: Vec<String> = env::args().collect();

    let name = args.get(0).unwrap().to_string();
    args.remove(0);

    let mut argv_m = false;
    let mut argv: Vec<String> = vec![];

    let (mut help, mut version) = (false, false);

    let (mut lex_out, mut prs_out) = (false, false);
    let (mut prd_out, mut outfile) = (false, None);

    if args.len() < 2 {
        if args.len() > 0 {
            for arg in &args {
                match arg.as_str() {
                    "-V" | "-s" | "--sürüm" => {
                        utilbin::print_version(name);
                    }
                    "-h" | "-y" | "--yardım" => utilbin::print_help(0, name),
                    "-L" | "--license" | "--lisans" => utilbin::print_license(),
                    _ => (),
                }
            }
        }
        utilbin::print_help(1, name)
    }

    let sub_cmd = match args.get(0).unwrap().as_str() {
        "y" | "yürüt" => Subcommands::Run,
        "b" | "byt" => Subcommands::Byt,
        "yb" | "yürbyt" => Subcommands::RunBytes,
        "k" | "komut" => Subcommands::Command,
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

    let mut outs = false;
    let mut license = false;

    let file = args.get(0)
        .expect("couldn't get <FILE>")
        .to_string();
    args = args[1..].to_vec();

    for arg in args {
        match arg.as_str() {
            a if argv_m => argv.push(a.to_string()),
            a if outs => {
                outs = false;
                outfile.replace(a.to_string());
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
            "--" => argv_m = true,
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
    }
}
