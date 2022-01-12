use crate::errwarn::ErrorGenerator;
use crate::util::{get_lang, SupportedLanguage};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct List {
    pub ls: Vec<Object>,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "{:?}", self.ls)?)
    }
}

#[derive(Clone)]
pub struct Map {
    pub map: HashMap<String, Object>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "{:?}", self.map)?)
    }
}

#[derive(Clone)]
pub enum Object {
    Sayı(f64),
    Yazı(String),
    Bool(bool),
    İşlev(usize),
    Liste(List),
    Harita(Map),
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sayı(n) => {
                if n.fract() == 0. {
                    write!(f, "{:.0?}", n)?
                } else {
                    write!(f, "{:?}", n)?
                }
            }
            Self::Bool(b) => match b {
                true => write!(f, "doğru")?,
                false => write!(f, "yanlış")?,
            },
            Self::Yazı(s) => write!(f, "{}", s)?,
            Self::İşlev(loc) => write!(f, "<işlev: {:?}>", loc)?,
            Self::Liste(ls) => write!(f, "{:?}", ls)?,
            Self::Harita(map) => write!(f, "{:?}", map)?,
        }
        Ok(())
    }
}

impl Object {
    // Karşılaştırma
    pub fn eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f == &a),
                b => panic!("{:?} `=` {:?} operatörü desteklemiyor", f, b),
            },
            Self::Bool(b) => match a {
                Self::Bool(a) => Self::Bool(b == &a),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", b, c),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(a) => Self::Bool(s == &a),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", s, c),
            },
            Self::İşlev(_) => unreachable!(),
            Self::Liste(l) => match a {
                Self::Liste(m) => Self::Bool(
                    l.ls.len() == m.ls.len()
                        && l.ls.iter().enumerate().all(|(k, v)| {
                            match v.eşittir(m.ls.get(k).unwrap().clone()) {
                                Self::Bool(b) => b,
                                _ => unreachable!(),
                            }
                        }),
                ),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", l, c),
            },
            Self::Harita(m) => match a {
                Self::Harita(n) => Self::Bool(
                    m.map.len() == n.map.len()
                        && m.map.keys().all(|k| {
                            n.map.contains_key(k)
                                && match m
                                    .map
                                    .get(k)
                                    .unwrap()
                                    .eşittir(n.map.get(k).unwrap().clone())
                                {
                                    Self::Bool(b) => b,
                                    _ => unreachable!(),
                                }
                        }),
                ),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", m, c),
            },
        }
    }
    pub fn eşit_değildir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f != &a),
                b => panic!("{:?} `!=` {:?} operatörü desteklemiyor", f, b),
            },
            Self::Bool(b) => match a {
                Self::Bool(a) => Self::Bool(b != &a),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", b, c),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(a) => Self::Bool(s != &a),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", s, c),
            },
            Self::İşlev(_) => unreachable!(),
            Self::Liste(l) => match a {
                Self::Liste(m) => Self::Bool(
                    l.ls.len() != m.ls.len()
                        || l.ls.iter().enumerate().any(|(k, v)| {
                            match v.eşit_değildir(m.ls.get(k).unwrap().clone()) {
                                Self::Bool(b) => b,
                                _ => unreachable!(),
                            }
                        }),
                ),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", l, c),
            },
            Self::Harita(m) => match a {
                Self::Harita(n) => Self::Bool(
                    m.map.len() != n.map.len()
                        || m.map.keys().any(|k| {
                            !n.map.contains_key(k)
                                || match m
                                    .map
                                    .get(k)
                                    .unwrap()
                                    .eşit_değildir(n.map.get(k).unwrap().clone())
                                {
                                    Self::Bool(b) => b,
                                    _ => unreachable!(),
                                }
                        }),
                ),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", m, c),
            },
        }
    }
    pub fn büyüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f > &a),
                b => panic!("{:?} `>` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>` operatörünü desteklemiyor", b),
        }
    }
    pub fn büyük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f >= &a),
                b => panic!("{:?} `>=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>=` operatörünü desteklemiyor", b),
        }
    }
    pub fn küçüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f < &a),
                b => panic!("{:?} `<` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    pub fn küçük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f <= &a),
                b => panic!("{:?} `<=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<=` operatörünü desteklemiyor", b),
        }
    }
    pub fn değildir(&self) -> Self {
        match self {
            Self::Bool(f) => Self::Bool(!f),
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    // Matematik
    pub fn ekle(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f + a),
                a => panic!("{:?} `+` {:?} desteklenmiyor", f, a),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(b) => {
                    let mut buf = String::new();
                    buf.push_str(s.as_str());
                    buf.push_str(b.as_str());
                    Self::Yazı(buf)
                }
                f => panic!("{:?} `+` {:?} desteklenmiyor", s, f),
            },
            Self::Liste(l) => match a {
                Self::Liste(mut m) => {
                    let mut l = l.clone();
                    l.ls.append(&mut m.ls);
                    Self::Liste(l)
                }
                f => panic!("{:?} `+` {:?} desteklenmiyor", l, f),
            },
            Self::Bool(b) => panic!("{:?} `+` operatörünü desteklemiyor", b),
            Self::Harita(m) => panic!("{:?} `+` operatörünü desteklemiyor", m),
            Self::İşlev(_) => unreachable!(),
        }
    }
    pub fn çıkar(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f - a),
                b => panic!("{:?} `-` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `-` operatörünü desteklemiyor", b),
        }
    }
    pub fn çarp(&self, a: Self) -> Self {
        match self {
            Self::Yazı(s) => {
                match a {
                    Self::Sayı(a) => {
                        let mut buf = String::new();
                        if a == (a as i128) as f64 {
                            for _ in 0..(a as i128) {
                                buf.push_str(s.as_str())
                            }
                        } else {
                            panic!("`*` operatörü tam olmayan sayılar ve yazılar arasında desteklenmiyor");
                        }
                        Self::Yazı(buf)
                    }
                    a => panic!("{:?} `*` {:?} desteklenmiyor", s, a),
                }
            }
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => Self::Sayı(f * a),
                    Self::Yazı(s) => {
                        let mut buf = String::new();
                        if f == &((*f as i128) as f64) {
                            for _ in 0..(*f as i128) {
                                buf.push_str(s.as_str())
                            }
                        } else {
                            panic!("`*` operatörü tam olmayan sayılar ve yazılar arasında desteklenmiyor");
                        }
                        Self::Yazı(buf)
                    }
                    b => panic!("{:?} `*` {:?} operatörü desteklemiyor", f, b),
                }
            }
            b => panic!("{:?} `*` operatörünü desteklemiyor", b),
        }
    }
    pub fn böl(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f / a),
                b => panic!("{:?} `/` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    pub fn modulo(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f % a),
                b => panic!("{:?} `/` {:?} desteklenmiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    // Mantık
    pub fn ve(&self, a: Self, line: usize, col: usize, file: String) -> Self {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Self::Bool(*f && a),
                b => match get_lang() {
                    SupportedLanguage::Turkish => {
                        ErrorGenerator::error(
                            "Desteklenmeyenİşlem",
                            &format!("`{:?}` `ve` `{:?}` işlemi desteklemiyor", self, b),
                            line,
                            col,
                            file,
                            Box::new(|| {}),
                        );
                    }
                    SupportedLanguage::English => {
                        ErrorGenerator::error(
                            "UnsupportedOperation",
                            &format!("`{:?}` `ve` `{:?}` operation is not supported", self, b),
                            line,
                            col,
                            file,
                            Box::new(|| {}),
                        );
                    }
                },
            },
            b => match get_lang() {
                SupportedLanguage::Turkish => {
                    ErrorGenerator::error(
                        "Desteklenmeyenİşlem",
                        &format!("{:?} `veya` anahtar kelimesini desteklemiyor", b),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
                SupportedLanguage::English => {
                    ErrorGenerator::error(
                        "UnsupportedOperation",
                        &format!("{:?} does not support the keyword `veya`", b),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
            },
        }
    }
    pub fn veya(&self, a: Self, line: usize, col: usize, file: String) -> Self {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Self::Bool(*f || a),
                b => match get_lang() {
                    SupportedLanguage::Turkish => {
                        ErrorGenerator::error(
                            "Desteklenmeyenİşlem",
                            &format!("`{:?}` `veya` `{:?}` işlemi desteklemiyor", self, b),
                            line,
                            col,
                            file,
                            Box::new(|| {}),
                        );
                    }
                    SupportedLanguage::English => {
                        ErrorGenerator::error(
                            "UnsupportedOperation",
                            &format!("`{:?}` `veya` `{:?}` operation is not supported", self, b),
                            line,
                            col,
                            file,
                            Box::new(|| {}),
                        );
                    }
                },
            },
            b => match get_lang() {
                SupportedLanguage::Turkish => {
                    ErrorGenerator::error(
                        "Desteklenmeyenİşlem",
                        &format!("{:?} `veya` anahtar kelimesini desteklemiyor", b),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
                SupportedLanguage::English => {
                    ErrorGenerator::error(
                        "UnsupportedOperation",
                        &format!("{:?} does not support the keyword `veya`", b),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
            },
        }
    }
    // Dönüşüm
    pub fn dönüştür(&self, a: String, line: usize, col: usize, file: String) -> Self {
        match a.to_lowercase().as_str() {
            "yazı" => match self {
                Self::Bool(b) => match b {
                    true => Self::Yazı("doğru".to_string()),
                    false => Self::Yazı("yanlış".to_string()),
                },
                Self::Sayı(n) => Self::Yazı(format!("{:?}", n)),
                Self::Yazı(_) => self.clone(),
                Self::İşlev(_) => unreachable!(),
                Self::Liste(l) => Self::Yazı(format!("{:?}", l)),
                Self::Harita(m) => Self::Yazı(format!("{:?}", m)),
            },
            "bool" | "boolean" => match self {
                Self::Bool(_) => self.clone(),
                Self::Sayı(n) => {
                    if n == &0. {
                        Self::Bool(false)
                    } else {
                        Self::Bool(true)
                    }
                }
                Self::Yazı(s) => match s.as_str() {
                    "doğru" => Self::Bool(true),
                    "yanlış" => Self::Bool(false),
                    _ => match get_lang() {
                        SupportedLanguage::Turkish => {
                            ErrorGenerator::error(
                                "DeğerHatası",
                                &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", s),
                                line,
                                col,
                                file,
                                Box::new(|| {}),
                            );
                        }
                        SupportedLanguage::English => {
                            ErrorGenerator::error(
                                "ValueError",
                                &format!("`{:?}` is not one of the expected values", s),
                                line,
                                col,
                                file,
                                Box::new(|| {}),
                            );
                        }
                    },
                },
                Self::İşlev(_) => unreachable!(),
                Self::Liste(_) | Self::Harita(_) => panic!("unsupported conversion"),
            },
            "sayı" => match self {
                Self::Bool(b) => match b {
                    true => Self::Sayı(1.),
                    false => Self::Sayı(0.),
                },
                Self::Sayı(_) => self.clone(),
                Self::Yazı(s) => match s.parse::<f64>() {
                    Ok(m) => Self::Sayı(m),
                    Err(_) => match get_lang() {
                        SupportedLanguage::Turkish => {
                            ErrorGenerator::error(
                                "DeğerHatası",
                                &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", s),
                                line,
                                col,
                                file,
                                Box::new(|| {}),
                            );
                        }
                        SupportedLanguage::English => {
                            ErrorGenerator::error(
                                "ValueError",
                                &format!("`{:?}` is not one of the expected values", s),
                                line,
                                col,
                                file,
                                Box::new(|| {}),
                            );
                        }
                    },
                },
                Self::İşlev(_) => unreachable!(),
                Self::Liste(_) | Self::Harita(_) => panic!("unsupported conversion"),
            },
            a => match get_lang() {
                SupportedLanguage::Turkish => {
                    ErrorGenerator::error(
                        "DeğerHatası",
                        &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", a),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
                SupportedLanguage::English => {
                    ErrorGenerator::error(
                        "ValueError",
                        &format!("`{:?}` is not one of the expected values", a),
                        line,
                        col,
                        file,
                        Box::new(|| {}),
                    );
                }
            },
        }
    }
}
