use crate::errwarn::ErrorGenerator;
use crate::token::{tokentypes::ParserTokenType as TokenType, ParserToken as Token};
use crate::util::{get_lang, SupportedLanguage};
use crate::store::SUPRESS_WARN;
use std::collections::HashMap;
use std::io::{self, prelude::*};

pub struct Run {
    program: Vec<Token>,
    current: usize,
}

#[derive(Clone)]
pub enum Object {
    Sayı(f64),
    Yazı(String),
    Bool(bool),
    İşlev(usize),
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sayı(n) => {
                if n == &((*n as i128) as f64) {
                    write!(f, "{:.0?}", n)
                } else {
                    write!(f, "{:?}", n)
                }
            }
            Self::Bool(b) => match b {
                true => write!(f, "doğru"),
                false => write!(f, "yanlış"),
            },
            Self::Yazı(s) => write!(f, "{}", s),
            Self::İşlev(loc) => write!(f, "<işlev: {:?}>", loc),
        }
    }
}

impl Object {
    // Karşılaştırma
    fn eşittir(&self, a: Self) -> Self {
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
        }
    }
    fn eşit_değildir(&self, a: Self) -> Self {
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
        }
    }
    fn büyüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f > &a),
                b => panic!("{:?} `>` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>` operatörünü desteklemiyor", b),
        }
    }
    fn büyük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f >= &a),
                b => panic!("{:?} `>=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `>=` operatörünü desteklemiyor", b),
        }
    }
    fn küçüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f < &a),
                b => panic!("{:?} `<` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    fn küçük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Bool(f <= &a),
                b => panic!("{:?} `<=` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `<=` operatörünü desteklemiyor", b),
        }
    }
    fn değildir(&self) -> Self {
        match self {
            Self::Bool(f) => Self::Bool(!f),
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    // Matematik
    fn ekle(&self, a: Self) -> Self {
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
            Self::Bool(b) => panic!("{:?} `+` operatörünü desteklemiyor", b),
            Self::İşlev(_) => unreachable!(),
        }
    }
    fn çıkar(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f - a),
                b => panic!("{:?} `-` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `-` operatörünü desteklemiyor", b),
        }
    }
    fn çarp(&self, a: Self) -> Self {
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
    fn böl(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f / a),
                b => panic!("{:?} `/` {:?} operatörü desteklemiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    fn modulo(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => match a {
                Self::Sayı(a) => Self::Sayı(f % a),
                b => panic!("{:?} `/` {:?} desteklenmiyor", f, b),
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    // Mantık
    fn ve(&self, a: Self) -> Self {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Self::Bool(*f && a),
                b => panic!("{:?} `ve` {:?} desteklenmiyor", f, b),
            },
            b => panic!("{:?} `ve` anahtar kelimesini desteklemiyor", b),
        }
    }
    fn veya(&self, a: Self) -> Self {
        match self {
            Self::Bool(f) => match a {
                Self::Bool(a) => Self::Bool(*f || a),
                b => panic!("{:?} `ve` {:?} desteklenmiyor", f, b),
            },
            b => panic!("{:?} `ve` anahtar kelimesini desteklemiyor", b),
        }
    }
    // Dönüşüm
    fn dönüştür(&self, a: String) -> Self {
        match a.to_lowercase().as_str() {
            "yazı" => match self {
                Self::Bool(b) => match b {
                    true => Self::Yazı("doğru".to_string()),
                    false => Self::Yazı("yanlış".to_string()),
                },
                Self::Sayı(n) => Self::Yazı(format!("{:?}", n)),
                Self::Yazı(_) => self.clone(),
                Self::İşlev(_) => unreachable!(),
            },
            "bool" | "boolean" => {
                match self {
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
                        _ => unimplemented!(), // SomeError
                    },
                    Self::İşlev(_) => unreachable!(),
                }
            }
            "sayı" => match self {
                Self::Bool(b) => match b {
                    true => Self::Sayı(1.),
                    false => Self::Sayı(0.),
                },
                Self::Sayı(_) => self.clone(),
                Self::Yazı(s) => match s.parse::<f64>() {
                    Ok(m) => Self::Sayı(m),
                    Err(_) => unimplemented!(),
                },
                Self::İşlev(_) => unreachable!(),
            },
            a => panic!("bilinmeyen tip: {}", a),
        }
    }
}

pub type Stack = Vec<Object>;

impl Run {
    pub fn new(program: Vec<Token>) -> Self {
        Self {
            program,
            current: 0,
        }
    }

    pub fn run(&mut self, file: String) {
        let mut stack: Stack = vec![];
        let mut hashs: HashMap<String, Object> = HashMap::new();
        // let mut warnings: Vec<Box<dyn Fn() -> ()>> = vec![]; // for later use
        let mut işlev_derinliği: usize = 0;

        while self.program.len() > self.current {
            let tokenc = self.program.get(self.current).unwrap().clone();
            let token = self.program.get_mut(self.current).unwrap();

            match token.typ.clone() {
                TokenType::İşlev { sonloc } => {
                    let id = self.program.get(self.current + 1).unwrap();
                    match id.typ.clone() {
                        TokenType::Identifier { id: ident } => {
                            hashs.insert(ident, Object::İşlev(self.current));
                        }
                        _ => unimplemented!(), // SyntaxError
                    }
                    let loc = match sonloc {
                        Some(a) => a,
                        None => unreachable!(),
                    };
                    match self.program.get_mut(loc).unwrap().typ {
                        TokenType::İşlevSonlandır { ref mut tp } => {
                            tp.push(loc);
                        }
                        _ => unreachable!(),
                    }
                    işlev_derinliği += 1;
                    self.current = loc;
                }
                TokenType::İşlevSonlandır { .. } => {
                    if işlev_derinliği < 1 {
                        let loc = match token.typ {
                            TokenType::İşlevSonlandır { ref mut tp } => tp.pop().unwrap(),
                            _ => unreachable!(),
                        };
                        self.current = loc;
                    } else {
                        işlev_derinliği -= 1;
                    }
                    self.current += 1;
                }
                TokenType::De => {
                    if işlev_derinliği < 1 {
                        print!("{:?}", stack.pop().unwrap());
                        io::stdout().flush().unwrap();
                    }
                    self.current += 1;
                }
                TokenType::Artı => {
                    if işlev_derinliği < 1 {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a.ekle(b));
                    }
                    self.current += 1;
                }
                TokenType::ArtıArtı => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        stack.push(a.ekle(Object::Sayı(1.0)));
                    }
                    self.current += 1;
                }
                TokenType::Eksi => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.çıkar(a));
                    }
                    self.current += 1;
                }
                TokenType::EksiEksi => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        stack.push(a.çıkar(Object::Sayı(1.0)));
                    }
                    self.current += 1;
                }
                TokenType::Çarpı => {
                    if işlev_derinliği < 1 {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a.çarp(b));
                    }
                    self.current += 1;
                }
                TokenType::Bölü => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.böl(a));
                    }
                    self.current += 1;
                }
                TokenType::Sayı { val } => {
                    if işlev_derinliği < 1 {
                        let n = Object::Sayı(val);
                        stack.push(n);
                    }
                    self.current += 1;
                }
                TokenType::Yazı { val } => {
                    if işlev_derinliği < 1 {
                        let s = Object::Yazı(val);
                        stack.push(s);
                    }
                    self.current += 1;
                }
                TokenType::Bool { val } => {
                    if işlev_derinliği < 1 {
                        let b = Object::Bool(val);
                        stack.push(b);
                        self.current += 1;
                    }
                }
                TokenType::İse(yoksa) | TokenType::İken(yoksa) => {
                    if işlev_derinliği < 1 {
                        if let Some(tp) = yoksa {
                            let a = stack.pop().unwrap();
                            match a {
                                Object::Bool(b) => {
                                    if b {
                                        self.current += 1;
                                    } else {
                                        self.current = tp;
                                    }
                                },
                                a => panic!("ise'den önce stackte bir boolean olması lazım; şu anda {:?} var", a),
                            }
                        } else {
                            unreachable!()
                        }
                    } else {
                        self.current += 1;
                    }
                }
                TokenType::Kopya => {
                    if işlev_derinliği < 1 {
                        let last = stack.pop().unwrap();
                        stack.push(last.clone());
                        stack.push(last);
                    }
                    self.current += 1;
                }
                TokenType::Büyüktür => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.büyüktür(a));
                    }
                    self.current += 1;
                }
                TokenType::BüyükEşittir => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.büyük_eşittir(a));
                    }
                    self.current += 1;
                }
                TokenType::Küçüktür => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.küçüktür(a));
                    }
                    self.current += 1;
                }
                TokenType::KüçükEşittir => {
                    if işlev_derinliği < 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b.küçük_eşittir(a));
                    }
                    self.current += 1;
                }
                TokenType::Eşittir => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(b.eşittir(a));
                    }
                    self.current += 1;
                }
                TokenType::EşitDeğildir => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(b.eşit_değildir(a));
                    }
                    self.current += 1;
                }
                TokenType::Değildir => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(a.değildir());
                    }
                    self.current += 1;
                }
                TokenType::Son { tp } => {
                    if işlev_derinliği < 1 {
                        self.current = tp;
                    } else {
                        self.current += 1;
                    }
                }
                TokenType::Yoksa(yoksa) => {
                    if işlev_derinliği < 1 {
                        if let Some(tp) = yoksa {
                            self.current = tp;
                        } else {
                            unreachable!()
                        }
                    } else {
                        self.current += 1;
                    }
                }
                TokenType::Modulo => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(b.modulo(a));
                    }
                    self.current += 1;
                }
                TokenType::Takas => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(a);
                        stack.push(b);
                    }
                    self.current += 1;
                }
                TokenType::Döndür => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let c = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(a);
                        stack.push(b);
                        stack.push(c);
                    }
                    self.current += 1;
                }
                TokenType::At => {
                    if işlev_derinliği < 1 {
                        match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                    }
                    self.current += 1;
                }
                TokenType::Üst => {
                    if işlev_derinliği < 1 {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        let b = match stack.pop() {
                            Some(a) => a,
                            None => match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                                }
                            },
                        };
                        stack.push(b.clone());
                        stack.push(a);
                        stack.push(b);
                    }
                    self.current += 1;
                }
                TokenType::Girdi => {
                    if işlev_derinliği < 1 {
                        let mut buf = String::new();
                        io::stdin().read_line(&mut buf).unwrap();
                        stack.push(Object::Yazı(buf.trim_end().to_string()));
                    }
                    self.current += 1;
                }
                TokenType::İkiNoktaNokta | TokenType::EOF => self.current += 1,
                TokenType::Identifier { id } => match hashs.get_mut(&id) {
                    Some(val) => match val {
                        Object::Bool(_) | Object::Sayı(_) | Object::Yazı(_) => {
                            stack.push(val.clone());
                            self.current += 1;
                        }
                        Object::İşlev(tp) => {
                            let işlev = self.program.get(*tp).unwrap();
                            match işlev.typ {
                                TokenType::İşlev { sonloc: tpi } => {
                                    let loc = match tpi {
                                        Some(i) => i,
                                        None => unreachable!(),
                                    };
                                    let işlevson = self.program.get_mut(loc).unwrap();
                                    match &mut işlevson.typ {
                                        TokenType::İşlevSonlandır { tp: ref mut tps } => {
                                            tps.push(self.current);
                                        }
                                        _ => unreachable!(),
                                    }
                                    self.current = *tp + 2;
                                }
                                _ => unreachable!(),
                            }
                        }
                    },
                    None => {
                        match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BilinmeyenTanımlayıcı",
                                    &format!("bilinmeyen değişken: `{}`, bu değişken bulunamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "UnknownIdentifier",
                                    &format!("unknown identifier: `{}`, this identifier could not be found", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{
                                        let mut hashc: Vec<String> = hashs.into_keys().collect();
                                        hashc.sort();
                                    }),
                                );
                            }
                        };
                    }
                },
                TokenType::Koy => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                        },
                    };
                    let id = self.program.get(self.current + 1).unwrap();
                    hashs.insert(match id.typ.clone() {
                        TokenType::Identifier { id : i } => i,
                        t => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BeklenmedikSimge",
                                    &format!("Tanımlayıcı simgesi beklenmişti ancak {:?} bulundu", t),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            },
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "BeklenmedikSimge",
                                    &format!("expected Identifier but found {:?}", t),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            },
                        },
                    }, a);
                    self.current += 2;
                }
                TokenType::Ve => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                        },
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                        },
                    };
                    stack.push(b.ve(a));
                    self.current += 1;
                }
                TokenType::Veya => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                        },
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    Box::new(||{}),
                                );
                            }
                        },
                    };
                    stack.push(b.veya(a));
                    self.current += 1;
                }
                TokenType::Tipinde => {
                    let a = stack.pop().unwrap();
                    self.current += 1;
                    let b = self.program.get_mut(self.current).unwrap();
                    match &b.typ {
                        TokenType::Identifier { id } => {
                            stack.push(a.dönüştür(id.clone()));
                        }
                        _ => unimplemented!("hata: tip tanımlayıcı değil"),
                    };
                    self.current += 1;
                }
            }
        }

        if stack.len() > 0 && !unsafe { SUPRESS_WARN } {
            match get_lang() {
                SupportedLanguage::Turkish => {
                    ErrorGenerator::warning(
                        "KümeBoşDeğil",
                        "küme boş değil, eğer nedeninden şüphe ediyorsanız kodunuzu kontrol etmeniz önerilir",
                        0,
                        0,
                        file
                    )();
                    print!("    kümede kalan değişkenler({:?}) [", stack.len());
                    for (i, o) in stack.iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Bool(_) | Object::Sayı(_) => format!("{:?}", o),
                            Object::İşlev(_) => unreachable!(),
                        };
                        if i > 0 {
                            print!(", {}", o);
                        } else {
                            if stack.len() > 3 {
                                print!("... {}", o);
                            } else {
                                print!("{}", o);
                            }
                        }
                    }
                    println!("]");
                }
                SupportedLanguage::English => {
                    ErrorGenerator::warning(
                        "StackNotEmpty",
                        "stack is not empty, if you aren't sure about why, you might want to take a look at you code",
                        0,
                        0,
                        file
                    )();
                    print!("    variables left in the stack({:?}) [", stack.len());
                    for (i, o) in stack.iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Bool(_) | Object::Sayı(_) => format!("{:?}", o),
                            Object::İşlev(_) => unreachable!(),
                        };
                        if i > 0 {
                            print!(", {}", o);
                        } else {
                            if stack.len() > 3 {
                                print!("... {}", o);
                            } else {
                                print!("{}", o);
                            }
                        }
                    }
                    println!("]");
                }
            }
        }
    }
}
