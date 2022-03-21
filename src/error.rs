use crate::util::{get_lang, SupportedLanguage};
use std::error;
use std::fmt;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Position {
    line: usize,
    col: usize,
    file: String,
    function_name: Option<String>
}
impl Position {
    pub fn new(line: usize, col: usize, file: String, function_name: Option<String>) -> Self {
        Self {
            line, col, file, function_name
        }
    }
}

#[derive(Debug)]
pub struct Error {
    name: String,
    explanation: String,
    traceback: Vec<Position>,
    after_note: Option<String>,
    is_warning: bool,
}
impl Error {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn explanation(&self) -> String {
        self.explanation.to_string()
    }
    pub fn traceback(&self) -> Vec<Position> {
        self.traceback.to_vec()
    }
    pub fn after_note(&self) -> Option<String> {
        self.after_note.to_owned()
    }
    pub fn is_warning(&self) -> bool {
        self.is_warning
    }

    pub fn error(&self) -> ! {
        self.error_print();
        exit(1);
    }
    pub fn warn(&self) {
        self.print(match get_lang() {
            SupportedLanguage::Turkish => ("UYARI", "Satır", "Sütun"),
            SupportedLanguage::English => ("WARNING", "Satır", "Sütun"),
        })
    }
    pub fn error_print(&self) {
        self.print(match get_lang() {
            SupportedLanguage::Turkish => ("HATA", "Satır", "Sütun"),
            SupportedLanguage::English => ("ERROR", "Line", "Column"),
        })
    }
    pub fn auto_print(&self) {
        if self.is_warning {
            self.warn();
        } else {
            self.error_print();
        }
    }
    pub fn auto(&self) {
        if self.is_warning {
            self.warn();
        } else {
            self.error();
        }
    }
    fn print(&self, (title, line, col): (&str, &str, &str)) {
        eprintln!();
        for position in &self.traceback {
            eprintln!(
                "[{}] {}, {} {:?}, {} {:?}{}",
                title, position.file, line, position.line, col, position.col,
                if let Some(a) = &position.function_name {
                    format!(": {}", a)
                } else { "".to_string() }
            );
        }
        eprintln!("    {}: {}", self.name, self.explanation);
        if let Some(note) = self.after_note.clone() {
            for line in note.lines() {
                eprintln!("    {line}");
            }
        }
    }
    pub fn new(
        name: &str,
        explanation: &str,
        traceback: Vec<(usize, usize, String, Option<String>)>,
        after_note: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            explanation: explanation.to_string(),
            traceback: traceback.into_iter()
                .map(|(line, col, file, fname)| Position::new(line, col, file, fname))
                .collect(),
            after_note,
            is_warning: false,
        }
    }
    pub fn warning(
        name: &str,
        explanation: &str,
        traceback: Vec<(usize, usize, String, Option<String>)>,
        after_note: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            explanation: explanation.to_string(),
            traceback: traceback.into_iter()
                .map(|(line, col, file, fname)| Position::new(line, col, file, fname))
                .collect(),
            after_note,
            is_warning: true,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for Error {}

