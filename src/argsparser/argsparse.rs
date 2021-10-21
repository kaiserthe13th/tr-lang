use std::env;

#[derive(Debug)]
pub enum Subcommands {
    Run,
    Byt,
    RunBytes,
}

#[derive(Debug)]
pub struct Options {
    pub name   :         String,
    pub file   :         String,
    pub outfile   : Option<String>,
    pub prd_out   :        bool,
    pub sub_cmd   : Subcommands,
    pub help      :        bool,
    pub help_exitc:         i32,
    pub version:           bool,
    pub lex_out:           bool,
    pub prs_out:           bool,
    pub argv   :    Vec<String>,
}

pub fn parse_args() -> Options {
    let mut args: Vec<String> = env::args().collect();

    let name = args.get(0).unwrap().to_string();
    args.remove(0);

    let mut help_exitc = 0;

    let mut argv_m = false;
    let mut argv: Vec<String> = vec![];

    let (mut help, mut version) = (false, false);

    let (mut lex_out, mut prs_out) = (false, false);
    let (mut prd_out, mut outfile) = (false, None);

    if args.len() < 2 {
        help_exitc = 1;
        return Options {
            name, help, version, argv, lex_out, prs_out, prd_out, outfile, file: "".to_string(), sub_cmd: Subcommands::Run, help_exitc
        }
    }

    let sub_cmd = match args.get(0).unwrap().as_str() {
        "y" | "yürüt" => Subcommands::Run,
        "b" | "byt" => Subcommands::Byt,
        "yb"| "yürbyt" => Subcommands::RunBytes,
        "-h" | "-y" | "--yardım" => {
            help = true;
            Subcommands::Run
        },
        "-V" | "-s" | "--sürüm" => {
            version = true;
            Subcommands::Run
        },
        a => crate::error_print("unknown subcommand", format!("{}", a)),
    };
    args.remove(0);

    let mut outs = false;

    let file = args.get(0).unwrap().to_string();
    args = args[1..].to_vec();
    
    for arg in args {
        match arg.as_str() {
            a if argv_m => argv.push(a.to_string()),
            a if outs => {
                outs = false;
                outfile.replace(a.to_string());
            },
            "-h" | "-y" | "--yardım" => help = true,
            "-V" | "-s" | "--sürüm" => version = true,
            "-l" | "--lexer-çıktısı" => lex_out = true,
            "-p" | "--parser-çıktısı" => prs_out = true,
            "-o" | "-ç" | "--çıkış" => {
                outs = true;
                prd_out = true;
            },
            "--" => argv_m = true,
            a => crate::error_print("unknown argument", format!("{}", a)),
        }
    }

    Options {
        name, help, version, argv, lex_out, prs_out, prd_out, outfile, file, sub_cmd, help_exitc
    }
}
