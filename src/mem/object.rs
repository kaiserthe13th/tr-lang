use crate::error::Error;
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

impl Map {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
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
    Hiç,
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
            Self::Hiç => write!(f, "hiç")?,
        }
        Ok(())
    }
}

type ObjectResult = Result<Object, Error>;

impl Object {
    // Karşılaştırma
    pub fn eşittir(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f == &a)),
                b => panic!("{:?} `=` {:?} operatörü desteklemiyor", f, b),
            },
            Self::Bool(b) => match a {
                Self::Bool(a) => Ok(Self::Bool(b == &a)),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", b, c),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(a) => Ok(Self::Bool(s == &a)),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", s, c),
            },
            Self::İşlev(_) => unreachable!(),
            Self::Liste(l) => match a {
                Self::Liste(m) => Ok(Self::Bool(
                    l.ls.len() == m.ls.len()
                        && l.ls.iter().enumerate().all(|(k, v)| {
                            match v.eşittir(m.ls.get(k).unwrap().clone()) {
                                Ok(Self::Bool(b)) => b,
                                _ => unreachable!(),
                            }
                        }),
                )),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", l, c),
            },
            Self::Harita(m) => match a {
                Self::Harita(n) => Ok(Self::Bool(
                    m.map.len() == n.map.len()
                        && m.map.keys().all(|k| {
                            n.map.contains_key(k)
                                && match m
                                    .map
                                    .get(k)
                                    .unwrap()
                                    .eşittir(n.map.get(k).unwrap().clone())
                                {
                                    Ok(Self::Bool(b)) => b,
                                    _ => panic!(),
                                }
                        }),
                )),
                c => panic!("{:?} `=` {:?} operatörü desteklemiyor", m, c),
            },
            Self::Hiç => match a {
                Self::Hiç => Ok(Self::Bool(true)),
                _ => Ok(Self::Bool(false)),
            },
        }
    }
    pub fn eşit_değildir(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f != &a)),
                b => panic!("{:?} `!=` {:?} operatörü desteklemiyor", f, b),
            },
            Self::Bool(b) => match a {
                Self::Bool(a) => Ok(Self::Bool(b != &a)),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", b, c),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(a) => Ok(Self::Bool(s != &a)),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", s, c),
            },
            Self::İşlev(_) => unreachable!(),
            Self::Liste(l) => match a {
                Self::Liste(m) => Ok(Self::Bool(
                    l.ls.len() != m.ls.len()
                        || l.ls.iter().enumerate().any(|(k, v)| {
                            match v.eşit_değildir(m.ls.get(k).unwrap().clone()) {
                                Ok(Self::Bool(b)) => b,
                                _ => unreachable!(),
                            }
                        }),
                )),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", l, c),
            },
            Self::Harita(m) => match a {
                Self::Harita(n) => Ok(Self::Bool(
                    m.map.len() != n.map.len()
                        || m.map.keys().any(|k| {
                            !n.map.contains_key(k)
                                || match m
                                    .map
                                    .get(k)
                                    .unwrap()
                                    .eşit_değildir(n.map.get(k).unwrap().clone())
                                {
                                    Ok(Self::Bool(b)) => b,
                                    _ => unreachable!(),
                                }
                        }),
                )),
                c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", m, c),
            },
            Self::Hiç => match a {
                Self::Hiç => Ok(Self::Bool(false)),
                _ => Ok(Self::Bool(true)),
            },
        }
    }
    pub fn büyüktür(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f > &a)),
                b => panic!("{:?} `>` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>` operatörünü desteklemiyor", b),
        }
    }
    pub fn büyük_eşittir(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f >= &a)),
                b => panic!("{:?} `>=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>=` operatörünü desteklemiyor", b),
        }
    }
    pub fn küçüktür(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f < &a)),
                b => panic!("{:?} `<` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    pub fn küçük_eşittir(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Bool(f <= &a)),
                b => panic!("{:?} `<=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<=` operatörünü desteklemiyor", b),
        }
    }
    pub fn değildir(&self) -> ObjectResult {
        match self {
            Self::Bool(f) => Ok(Self::Bool(!f)),
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    // Matematik
    pub fn ekle(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Sayı(f + a)),
                a => panic!("{:?} `+` {:?} desteklenmiyor", f, a),
            },
            Self::Yazı(s) => match a {
                Self::Yazı(b) => {
                    let mut buf = String::new();
                    buf.push_str(s.as_str());
                    buf.push_str(b.as_str());
                    Ok(Self::Yazı(buf))
                }
                f => panic!("{:?} `+` {:?} desteklenmiyor", s, f),
            },
            Self::Liste(l) => match a {
                Self::Liste(mut m) => {
                    let mut l = l.clone();
                    l.ls.append(&mut m.ls);
                    Ok(Self::Liste(l))
                }
                f => panic!("{:?} `+` {:?} desteklenmiyor", l, f),
            },
            Self::Bool(b) => panic!("{:?} `+` operatörünü desteklemiyor", b),
            Self::Harita(m) => panic!("{:?} `+` operatörünü desteklemiyor", m),
            Self::İşlev(_) => unreachable!(),
            Self::Hiç => panic!("hiç `+` operatörünü desteklemiyor"),
        }
    }
    pub fn çıkar(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Sayı(f - a)),
                b => panic!("{:?} `-` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `-` operatörünü desteklemiyor", b),
        }
    }
    pub fn çarp(&self, a: Self) -> ObjectResult {
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
                        Ok(Self::Yazı(buf))
                    }
                    a => panic!("{:?} `*` {:?} desteklenmiyor", s, a),
                }
            }
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => Ok(Self::Sayı(f * a)),
                    Self::Yazı(s) => {
                        let mut buf = String::new();
                        if f == &((*f as i128) as f64) {
                            for _ in 0..(*f as i128) {
                                buf.push_str(s.as_str())
                            }
                        } else {
                            panic!("`*` operatörü tam olmayan sayılar ve yazılar arasında desteklenmiyor");
                        }
                        Ok(Self::Yazı(buf))
                    }
                    b => panic!("{:?} `*` {:?} operatörü desteklemiyor", f, b),
                }
            }
            b => panic!("{:?} `*` operatörünü desteklemiyor", b),
        }
    }
    pub fn böl(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Sayı(f / a)),
                b => panic!("{:?} `/` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    pub fn modulo(&self, a: Self) -> ObjectResult {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Ok(Self::Sayı(f % a)),
                b => panic!("{:?} `/` {:?} desteklenmiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    // Mantık
    pub fn ve(&self, a: Self, line: usize, col: usize, file: String) -> ObjectResult {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Ok(Self::Bool(*f && a)),
                b => match get_lang() {
                    SupportedLanguage::Turkish => Err(Error::new(
                        "Desteklenmeyenİşlem",
                        &format!("`{:?}` `ve` `{:?}` işlemi desteklemiyor", self, b),
                        vec![(line,
                        col,
                        file, None)],
                        None,
                    )),
                    SupportedLanguage::English => Err(Error::new(
                        "UnsupportedOperation",
                        &format!("`{:?}` `ve` `{:?}` operation is not supported", self, b),
                        vec![(line,
                        col,
                        file, None)],
                        None,
                    )),
                },
            },
            b => match get_lang() {
                SupportedLanguage::Turkish => Err(Error::new(
                    "Desteklenmeyenİşlem",
                    &format!("{:?} `veya` anahtar kelimesini desteklemiyor", b),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                )),
                SupportedLanguage::English => Err(Error::new(
                    "UnsupportedOperation",
                    &format!("{:?} does not support the keyword `veya`", b),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                )),
            },
        }
    }
    pub fn veya(&self, a: Self, line: usize, col: usize, file: String) -> ObjectResult {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Ok(Self::Bool(*f || a)),
                b => match get_lang() {
                    SupportedLanguage::Turkish => Err(Error::new(
                        "Desteklenmeyenİşlem",
                        &format!("`{:?}` `veya` `{:?}` işlemi desteklemiyor", self, b),
                        vec![(line,
                        col,
                        file, None)],
                        None,
                    )),
                    SupportedLanguage::English => Err(Error::new(
                        "UnsupportedOperation",
                        &format!("`{:?}` `veya` `{:?}` operation is not supported", self, b),
                        vec![(line,
                        col,
                        file, None)],
                        None,
                    )),
                },
            },
            b => match get_lang() {
                SupportedLanguage::Turkish => Err(Error::new(
                    "Desteklenmeyenİşlem",
                    &format!("{:?} `veya` anahtar kelimesini desteklemiyor", b),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                )),
                SupportedLanguage::English => Err(Error::new(
                    "UnsupportedOperation",
                    &format!("{:?} does not support the keyword `veya`", b),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                )),
            },
        }
    }
    // Dönüşüm
    pub fn dönüştür(&self, a: String, line: usize, col: usize, file: String) -> ObjectResult {
        match a.to_lowercase().as_str() {
            "yazı" => match self {
                Self::Bool(b) => match b {
                    true => Ok(Self::Yazı("doğru".to_string())),
                    false => Ok(Self::Yazı("yanlış".to_string())),
                },
                Self::Sayı(n) => Ok(Self::Yazı(if n.fract() == 0. { format!("{:.0?}", n) } else { format!("{:?}", n) })),
                Self::Yazı(_) => Ok(self.clone()),
                Self::İşlev(_) => unreachable!(),
                Self::Liste(l) => Ok(Self::Yazı(format!("{:?}", l))),
                Self::Harita(m) => Ok(Self::Yazı(format!("{:?}", m))),
                Self::Hiç => Ok(Self::Yazı("hiç".to_string())),
            },
            "bool" | "boolean" => match self {
                Self::Bool(_) => Ok(self.clone()),
                Self::Hiç => Ok(Self::Bool(false)),
                Self::Sayı(n) => Ok(if n == &0. {
                    Self::Bool(false)
                } else {
                    Self::Bool(true)
                }),
                Self::Yazı(s) => match s.as_str() {
                    "doğru" => Ok(Self::Bool(true)),
                    "yanlış" => Ok(Self::Bool(false)),
                    _ => Err(match get_lang() {
                        SupportedLanguage::Turkish => Error::new(
                            "DeğerHatası",
                            &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", s),
                            vec![(line,
                            col,
                            file, None)],
                            None,
                        ),
                        SupportedLanguage::English => Error::new(
                            "ValueError",
                            &format!("`{:?}` is not one of the expected values", s),
                            vec![(line,
                            col,
                            file, None)],
                            None,
                        ),
                    }),
                },
                Self::İşlev(_) => unreachable!(),
                Self::Liste(_) | Self::Harita(_) => panic!("unsupported conversion"),
            },
            "sayı" => match self {
                Self::Bool(b) => Ok(match b {
                    true => Self::Sayı(1.),
                    false => Self::Sayı(0.),
                }),
                Self::Hiç => Ok(Self::Sayı(0.)),
                Self::Sayı(_) => Ok(self.clone()),
                Self::Yazı(s) => match s.parse::<f64>() {
                    Ok(m) => Ok(Self::Sayı(m)),
                    Err(_) => Err(match get_lang() {
                        SupportedLanguage::Turkish => Error::new(
                            "DeğerHatası",
                            &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", s),
                            vec![(line,
                            col,
                            file, None)],
                            None,
                        ),
                        SupportedLanguage::English => Error::new(
                            "ValueError",
                            &format!("`{:?}` is not one of the expected values", s),
                            vec![(line,
                            col,
                            file, None)],
                            None,
                        ),
                    }),
                },
                Self::İşlev(_) => unreachable!(),
                Self::Liste(_) | Self::Harita(_) => panic!("unsupported conversion"),
            },
            a => Err(match get_lang() {
                SupportedLanguage::Turkish => Error::new(
                    "DeğerHatası",
                    &format!("`{:?}` beklenen değerlerin arasında bulunmuyor", a),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                ),
                SupportedLanguage::English => Error::new(
                    "ValueError",
                    &format!("`{:?}` is not one of the expected values", a),
                    vec![(line,
                    col,
                    file, None)],
                    None,
                ),
            }),
        }
    }
}
