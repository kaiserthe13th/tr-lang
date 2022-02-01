use std::process::exit;
use std::fs::File;
use std::io::ErrorKind::IsADirectory;
use std::io::Read;
use std::path::PathBuf;

use locale_config::Locale;

#[macro_export]
macro_rules! hashmap {
    {} => { std::collections::HashMap::new() };
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

