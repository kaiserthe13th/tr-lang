#[allow(non_snake_case)]
pub mod ErrorGenerator {
    use crate::exit;
    use crate::store::SUPRESS_WARN;
    use crate::util::{ get_lang, SupportedLanguage };

    pub fn error(name: &str, explanation: &str, line: usize, col: usize, file: String) -> ! {
        match get_lang() {
            SupportedLanguage::English => {
                eprintln!("\n[ERROR] {}, Line {:?}, Column {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
            SupportedLanguage::Turkish => {
                eprintln!("\n[HATA] {}, Satır {:?}, Sütun {:?}", file, line, col);
                eprintln!("    {}: {}", name, explanation);
            },
        }
        exit(1);
    }
    pub fn warning(name: &'static str, explanation: &'static str, line: usize, col: usize, file: String) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move | | {
            if !unsafe{ SUPRESS_WARN } {
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
        })
    }
}
