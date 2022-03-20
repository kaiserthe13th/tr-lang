pub const LICENSE: &str = include_str!("../LICENSE");
pub const RELEASE: &str = env!("RELEASE_DATE");
pub use crate::store::VERSION;

use crate::exit;
use crate::util::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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
            #[cfg(feature = "interactive")]
            println!("    i inter    interaktif konsolu aç");
            #[cfg(feature = "fmt")]
            println!("    f fmt      programı formatla (deneysel)");
            println!("");
            println!("SEÇENEKLER:");
            println!("    -h -y --yardım         yardım göster ve çık");
            println!("    -V -s --sürüm          sürümü göster ve çık");
            println!("    -o -ç --çıkış <DOSYA>  çıkışta buraya bytecode yaz");
            println!("    -l --lexer-çıktısı     lex sürecinden sonra lexer'ın çıktısını göster");
            #[cfg(feature = "interactive")]
            println!(
                "    -p --parser-çıktısı    parse sürecinden sonra parser'ın çıktısını göster"
            );
            #[cfg(feature = "interactive")]
            println!("    -q --sessiz            1 kadar sessizleştir [0]");
            #[cfg(feature = "fmt")]
            println!("    -i --indent --girinti  format için girinti ver {{ tabs | [space sayısı] }}");
            #[cfg(feature = "fmt")]
            println!("    --lending --satson     format için satır sonu ver {{ lf | crlf }}");
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
            #[cfg(feature = "interactive")]
            println!("    i inter    open interactive console");
            #[cfg(feature = "fmt")]
            println!("    f fmt      format program (experimental)");
            println!("");
            println!("OPTIONS:");
            println!("    -h -y --yardım         print help and exit");
            println!("    -V -s --sürüm          print version and exit");
            println!("    -o -ç --çıkış <file>   write bytecode at <file>");
            println!("    -l --lexer-çıktısı     after lexing show lexed tokens");
            println!("    -p --parser-çıktısı    after parsing show parsed tokens");
            #[cfg(feature = "interactive")]
            println!("    -q --sessiz            quiet the output by 1 [0]");
            #[cfg(feature = "fmt")]
            println!("    -i --indent --girinti  give indentation for format {{ tabs | [number of spaces] }}");
            #[cfg(feature = "fmt")]
            println!("    --lending --satson     give line ending for format {{ lf | crlf }}");
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
