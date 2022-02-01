use crate::util::{get_lang, SupportedLanguage};
use std::process::exit;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    name: String,
    explanation: String,
    position: (usize, usize, String),
    after_note: Option<String>,
}
impl Error {
    pub fn error(&self) -> ! {
        self.eprint();
        exit(1);
    }
    pub fn eprint(&self) {
        match get_lang() {
            SupportedLanguage::English => {
                eprintln!("\n[ERROR] {}, Line {:?}, Column {:?}", self.position.2, self.position.0, self.position.1);
                eprintln!("    {}: {}", self.name, self.explanation);
            }
            SupportedLanguage::Turkish => {
                eprintln!("\n[HATA] {}, Satır {:?}, Sütun {:?}", self.position.2, self.position.0, self.position.1);
                eprintln!("    {}: {}", self.name, self.explanation);
            }
        }
        if let Some(note) = self.after_note.clone() {
            for line in note.lines() {
                println!("    {line}");
            }
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}

#[allow(non_snake_case)]
pub mod ErrorGenerator {
    use super::Error;
    use crate::store::globarg::SUPRESS_WARN;
    use crate::util::{get_lang, SupportedLanguage};

    pub fn error(
        name: &str,
        explanation: &str,
        line: usize,
        col: usize,
        file: String,
        after_note: Option<String>,
    ) -> Error {
        Error {
            name: name.to_string(),
            explanation: explanation.to_string(),
            position: (line, col, file),
            after_note,
        }
    }
    pub fn warning(
        name: &'static str,
        explanation: &'static str,
        line: usize,
        col: usize,
        file: String,
    ) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move || {
            if !unsafe { SUPRESS_WARN } {
                match get_lang() {
                    SupportedLanguage::English => {
                        eprintln!("[WARNING] {}, Line {:?}, Column {:?}", file, line, col);
                        eprintln!("    {}: {}", name, explanation);
                    }
                    SupportedLanguage::Turkish => {
                        eprintln!("[UYARI] {}, Satır {:?}, Sütun {:?}", file, line, col);
                        eprintln!("    {}: {}", name, explanation);
                    }
                }
            }
        })
    }
}
