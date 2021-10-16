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
    println!("tr-lang version {} released at {}", VERSION, RELEASE);
    println!("");
    println!("USAGE:");
    println!("  {} <FILE> [options]", prog_name);
    println!("");
    println!("OPTIONS:");
    println!("  -h --help     print help and exit");
    println!("  -v --version  print version and exit");
    crate::exit(exit_code);
}

pub fn print_version(prog_name: String) -> ! {
    println!("{} cersion {} released at {}", prog_name, VERSION, RELEASE);
    crate::exit(0);
}