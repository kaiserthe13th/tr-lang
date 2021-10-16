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