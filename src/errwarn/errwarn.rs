use crate::exit;
use crate::util::{ get_lang, SupportedLanguage };

pub struct ErrorGenerator();

impl ErrorGenerator {
    fn new() -> Self {Self()}
    fn error(name: String, explanation: String, line: usize, col: usize, file: String) -> ! {
        match get_lang() {
            SupportedLanguage::English => {
                eprintln!("[ERROR] {}, Line {:?}, Column {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
            SupportedLanguage::Turkish => {
                eprintln!("[HATA] {}, Satır {:?}, Sütun {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
        }
        exit(1);
    }
    fn warning(name: String, explanation: String, line: usize, col: usize, file: String) {
        match get_lang() {
            SupportedLanguage::English => {
                eprintln!("[WARNING] {}, Line {:?}, Column {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
            SupportedLanguage::Turkish => {
                eprintln!("[UYARI] {}, Satır {:?}, Sütun {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
        }
    }
}
