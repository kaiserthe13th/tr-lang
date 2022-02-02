pub const LICENSE: &str = include_str!("../LICENSE");
pub const RELEASE: &str = env!("RELEASE_DATE");
pub use crate::store::VERSION;

use crate::exit;
use std::path::PathBuf;
use crate::util::*;
use std::fs::File;
use std::io::Read;

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

/* pub fn item_in_vec<T>(arr: &[T], v: &Vec<T>) -> bool
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
} */

pub fn print_help(exit_code: i32, prog_name: String) -> ! {
    match get_lang() {
        SupportedLanguage::Turkish => {
            println!(
                "{} sürüm {}, {} tarihinde yayınlandı",
                prog_name, VERSION, RELEASE
            );
            println!("");
            println!("KULLANIM:");
            println!("  {} <ALTKOMUT> <DOSYA|KOMUT> [SEÇENEKLER]", prog_name);
            println!("");
            println!("ALTKOMUTLAR:");
            println!("    y yürüt    DOSYA'yı yürüt");
            println!("    b byt      DOSYA'yı bytecode'a dönüstür");
            println!("    yb yürbyt  bytecode DOSYA'sını yürüt");
            println!("    k komut    KOMUT'u yürüt");
            println!("");
            println!("SEÇENEKLER:");
            println!("    -h -y --yardım         yardım göster ve çık");
            println!("    -V -s --sürüm          sürümü göster ve çık");
            println!("    -o -ç --çıkış <DOSYA>  çıkışta buraya bytecode yaz");
            println!("    -l --lexer-çıktısı     lex sürecinden sonra lexer'ın çıktısını göster");
            println!(
                "    -p --parser-çıktısı    parse sürecinden sonra parser'ın çıktısını göster"
            );
            println!("    -L --license --lisans  projenin lisansını göster ve çık");
            println!("    --                     bundan sonra argv ekleyin");
        }
        SupportedLanguage::English => {
            println!("{} version {}, released at {}", prog_name, VERSION, RELEASE);
            println!("");
            println!("USAGE:");
            println!("    {} <SUBCOMMAND> <FILE|CMD> [OPTIONS]", prog_name);
            println!("");
            println!("SUBCOMMANDS:");
            println!("    y yürüt    run FILE");
            println!("    b byt      output bytecode for FILE");
            println!("    yb yürbyt  run bytecode FILE");
            println!("    k komut    run CMD");
            println!("");
            println!("OPTIONS:");
            println!("    -h -y --yardım         print help and exit");
            println!("    -V -s --sürüm          print version and exit");
            println!("    -o -ç --çıkış <file>   write bytecode at <file>");
            println!("    -l --lexer-çıktısı     after lexing show lexed tokens");
            println!("    -p --parser-çıktısı    after parsing show parsed tokens");
            println!("    -L --license --lisans  print license and exit");
            println!("    --                     add argv after this");
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

pub fn print_license() -> ! {
    println!("{}", LICENSE);
    exit(0);
}

