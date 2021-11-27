use crate::exit;
use crate::store::{RELEASE, VERSION};

use std::fs::File;
use std::io::ErrorKind::IsADirectory;
use std::io::Read;
use std::path::PathBuf;

use locale_config::Locale;

#[macro_export]
macro_rules! hashmap {
    { $({$k:expr => $v:expr}),* } => {
        {
            #[allow(unused_mut)]
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
}

#[derive(Debug)]
pub enum FSErr {
    IsADir,
}

pub enum SupportedLanguage {
    Turkish,
    English,
}

pub fn error_print<T>(error_name: &str, error_explanation: T) -> !
where
    T: std::fmt::Debug,
{
    eprintln!("{}: {:?}", error_name, error_explanation);
    exit(1);
}

pub fn read_file(path: &PathBuf) -> Result<String, FSErr> {
    let mut file = match File::open(path.clone()) {
        Err(e) => error_print(
            "error opening file",
            format!("{}: {}", e, path.as_path().display()),
        ),
        Ok(f) => f,
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(err) => match err.kind() {
            IsADirectory => {
                return Err(FSErr::IsADir);
            }
            _ => panic!(),
        },
    };
    Ok(buf)
}

pub fn read_file_to_vec_u8(path: &PathBuf) -> Vec<u8> {
    let mut file = match File::open(path.clone()) {
        Err(e) => error_print(
            "error opening file",
            format!("{}: {}", e, path.as_path().display()),
        ),
        Ok(f) => f,
    };

    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();
    buf
}

pub fn get_lang() -> SupportedLanguage {
    //! Returns a SupportedLanguage by checking systems default locale
    //! if an unrecognized language is found it will return English
    let l = format!("{}", Locale::user_default());
    // As Unix provides more info about locale(number format, date format, time format...)
    // separated by ',' we split once at the first ',' if it is successfull we take the first
    // else we just retain former state
    let lang = match l.split_once(',') {
        None => l,
        Some((a, _)) => a.to_string(),
    };
    match lang.as_str() {
        "tr-TR" => SupportedLanguage::Turkish,
        _ => SupportedLanguage::English,
    }
}

pub fn char_in_str(a: char, b: &str) -> bool {
    for ch in b.chars() {
        if ch == a {
            return true;
        }
    }
    false
}

pub fn in_vec<T>(a: &T, v: &Vec<T>) -> bool
where
    T: Eq,
{
    //! Checks if &T is in a &Vec<T>
    //! It is shortcircuiting function meaning if it finds any match it will immideatly return true
    //! if no matches are found it will return false
    for item in v.iter() {
        if item == a {
            return true;
        }
    }
    false
}

pub fn item_in_vec<T>(arr: &[T], v: &Vec<T>) -> bool
where
    T: Eq,
{
    //! Checks if any item of arr: &[T] has a counterpart in v: &Vec<T>
    //! It is shortcircuiting function meaning if it finds any match it will immideatly return true
    //! if no matches are found it will return false
    for a in arr {
        for item in v.iter() {
            if item == a {
                return true;
            }
        }
    }
    false
}

pub fn print_help(exit_code: i32, prog_name: String) -> ! {
    match get_lang() {
        SupportedLanguage::Turkish => {
            println!(
                "{} sürüm {}, {} tarihinde yayınlandı",
                prog_name, VERSION, RELEASE
            );
            println!("");
            println!("KULLANIM:");
            println!("  {} <ALTKOMUT> <DOSYA> [SEÇENEKLER]", prog_name);
            println!("");
            println!("ALTKOMUTLAR:");
            println!("    y yürüt    DOSYA'yı yürüt");
            println!("    b byt      DOSYA'yı bytecode'a dönüstür");
            println!("    yb yürbyt  bytecode DOSYA'sını yürüt");
            println!("");
            println!("SEÇENEKLER:");
            println!("  -h -y --yardım         yardım göster ve çık");
            println!("  -V -s --sürüm          sürümü göster ve çık");
            println!("  -o -ç --çıkış <DOSYA>  çıkışta buraya bytecode yaz");
            println!("  -l --lexer-çıktısı     lex sürecinden sonra lexer'ın çıktısını göster");
            println!("  -p --parser-çıktısı    parse sürecinden sonra parser'ın çıktısını göster");
            println!("  --                     bundan sonra argv ekleyin");
        }
        SupportedLanguage::English => {
            println!("{} version {}, released at {}", prog_name, VERSION, RELEASE);
            println!("");
            println!("USAGE:");
            println!("    {} <SUBCOMMAND> <FILE> [OPTIONS]", prog_name);
            println!("");
            println!("SUBCOMMANDS:");
            println!("    y yürüt    run FILE");
            println!("    b byt      output bytecode for FILE");
            println!("    yb yürbyt  run bytecode FILE");
            println!("");
            println!("OPTIONS:");
            println!("    -h -y --yardım        print help and exit");
            println!("    -V -s --sürüm         print version and exit");
            println!("    -o -ç --çıkış <file>  write bytecode at <file>");
            println!("    -l --lexer-çıktısı    after lexing show lexed tokens");
            println!("    -p --parser-çıktısı   after parsing show parsed tokens");
            println!("    --                    add argv after this");
        }
    }
    exit(exit_code);
}

pub fn print_version(prog_name: String) -> ! {
    match get_lang() {
        SupportedLanguage::Turkish => println!(
            "{} sürüm {}, {} tarihinde yayınlandı",
            prog_name, VERSION, RELEASE
        ),
        SupportedLanguage::English => {
            println!("{} version {}, released at {}", prog_name, VERSION, RELEASE)
        }
    }
    exit(0);
}
