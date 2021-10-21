use crate::store::{
    VERSION,
    RELEASE,
};

pub fn char_in_str(a: char, b: &str) -> bool {
    for ch in b.chars() {
        if ch == a {
            return true;
        }
    }
    false
}

pub fn in_vec<T>(a: &T, v: &Vec<T>) -> bool
where T: Eq
{
    for item in v.iter() {
        if item == a {
            return true;
        }
    }
    false
}

pub fn item_in_vec<T>(arr: &[T], v: &Vec<T>) -> bool
where T: Eq
{
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
    println!("{} sürüm {}, {} tarihinde yayınlandı", prog_name, VERSION, RELEASE);
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
    println!("  -l --lexer-çıktısı     lex süreci bittikten sonra lexer'ın çıktısını göster");
    println!("  -p --parser-çıktısı    parse süreci bittikten sonra parser'ın çıktısını göster");
    println!("  --                     bundan sonra argv ekleyin");
    crate::exit(exit_code);
}

pub fn print_version(prog_name: String) -> ! {
    println!("{} sürüm {}, {} tarihinde yayınlandı", prog_name, VERSION, RELEASE);
    crate::exit(0);
}
