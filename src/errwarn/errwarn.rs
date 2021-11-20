#[allow(non_snake_case)]
pub mod ErrorGenerator {
    use crate::exit;
    use crate::util::{ get_lang, SupportedLanguage };

    pub fn error(name: &str, explanation: &str, line: usize, col: usize, file: String) -> ! {
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
    pub fn warning(name: &'static str, explanation: &'static str, line: usize, col: usize, file: String) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move | | {
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
        })
    }
}
