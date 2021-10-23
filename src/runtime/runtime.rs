use crate::token::{ ParserToken as Token, tokentypes::ParserTokenType as TokenType };
use std::io::{ self, prelude::* };

pub struct Run {
    program: Vec<Token>,
    current: usize,
}

#[derive(Clone)]
pub enum Object {
    Sayı(f64),
    Yazı(String),
    Bool(bool),
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sayı(n) => {
                if n == &((*n as usize) as f64) {
                    write!(f, "{:.0?}", n)
                } else {
                    write!(f, "{:?}", n)
                }
            },
            Self::Bool(b) => write!(f, "{:?}", b),
            Self::Yazı(s) => write!(f, "{}", s),
        }
    }
}

impl Object {
    // Karşılaştırma
    fn eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f==&a)
                    },
                    b => panic!("{:?} `=` {:?} operatörü desteklemiyor", f, b),
                }
            },
            Self::Bool(b) => {
                match a {
                    Self::Bool(a) => {
                        Self::Bool(b==&a)
                    },
                    c => panic!("{:?} `=` {:?} operatörü desteklemiyor", b, c),
                }
            },
            Self::Yazı(s) => {
                match a {
                    Self::Yazı(a) => {
                        Self::Bool(s==&a)
                    },
                    c => panic!("{:?} `=` {:?} operatörü desteklemiyor", s, c),
                }
            },
        }
    }
    fn eşit_değildir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f!=&a)
                    },
                    b => panic!("{:?} `!=` {:?} operatörü desteklemiyor", f, b),
                }
            },
            Self::Bool(b) => {
                match a {
                    Self::Bool(a) => {
                        Self::Bool(b!=&a)
                    },
                    c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", b, c),
                }
            },
            Self::Yazı(s) => {
                match a {
                    Self::Yazı(a) => {
                        Self::Bool(s!=&a)
                    },
                    c => panic!("{:?} `!=` {:?} operatörü desteklemiyor", s, c),
                }
            },
        }
    }
    fn büyüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f>&a)
                    },
                    b => panic!("{:?} `>` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `>` operatörünü desteklemiyor", b),
        }
    }
    fn büyük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f>=&a)
                    },
                    b => panic!("{:?} `>=` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `>=` operatörünü desteklemiyor", b),
        }
    }
    fn küçüktür(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f<&a)
                    },
                    b => panic!("{:?} `<` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `<` operatörünü desteklemiyor", b),
        }
    }
    fn küçük_eşittir(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Bool(f<=&a)
                    },
                    b => panic!("{:?} `<=` {:?} operatörü desteklemiyor", f, b),
                }
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
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Sayı(f+a)
                    },
                    a => panic!("{:?} `+` {:?} desteklenmiyor", f, a),
                }
            },
            Self::Yazı(s) => {
                match a {
                    Self::Yazı(b) => {
                        let mut buf = String::new();
                        buf.push_str(s.as_str());
                        buf.push_str(b.as_str());
                        Self::Yazı(buf)
                    },
                    f => panic!("{:?} `+` {:?} desteklenmiyor", s, f),
                }
            },
            Self::Bool(b) => panic!("{:?} `+` operatörünü desteklemiyor", b),
        }
    }
    fn çıkar(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Sayı(f-a)
                    },
                    b => panic!("{:?} `-` {:?} operatörü desteklemiyor", f, b),
                }
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
                        if a == (a as usize) as f64 {
                            for _ in 0..(a as usize) {
                                buf.push_str(s.as_str())
                            }
                        } else {
                            panic!("`*` operatörü tam olmayan sayılar ve yazılar arasında desteklenmiyor");
                        }
                        Self::Yazı(buf)
                    },
                    a => panic!("{:?} `*` {:?} desteklenmiyor", s, a),
                }
            },
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Sayı(f*a)
                    },
                    Self::Yazı(s) => {
                        let mut buf = String::new();
                        if f == &((*f as usize) as f64) {
                            for _ in 0..(*f as usize) {
                                buf.push_str(s.as_str())
                            }
                        } else {
                            panic!("`*` operatörü tam olmayan sayılar ve yazılar arasında desteklenmiyor");
                        }
                        Self::Yazı(buf)
                    }
                    b => panic!("{:?} `*` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `*` operatörünü desteklemiyor", b),
        }
    }
    fn böl(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Sayı(f/a)
                    },
                    b => panic!("{:?} `/` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
    fn modulo(&self, a: Self) -> Self {
        match self {
            Self::Sayı(f) => {
                match a {
                    Self::Sayı(a) => {
                        Self::Sayı(f%a)
                    },
                    b => panic!("{:?} `/` {:?} operatörü desteklemiyor", f, b),
                }
            },
            b => panic!("{:?} `/` operatörünü desteklemiyor", b),
        }
    }
}

pub type Stack = Vec<Object>;

impl Run {
    pub fn new(program: Vec<Token>) -> Self {
        Self { program, current: 0 }
    }

    pub fn run(&mut self) {
        let mut stack: Stack = vec![];
        while self.program.len() > self.current {
            let token = self.program.get(self.current).unwrap();
            
            match token.typ.clone() {
                TokenType::De => {
                    print!("{:?}", stack.pop().unwrap());
                    io::stdout().flush().unwrap();
                    self.current += 1;
                },
                TokenType::Artı => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a.ekle(b));
                    self.current += 1;
                },
                TokenType::ArtıArtı => {
                    let a = stack.pop().unwrap();
                    stack.push(a.ekle(Object::Sayı(1.0)));
                    self.current += 1;
                },
                TokenType::Eksi => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.çıkar(a));
                    self.current += 1;
                },
                TokenType::EksiEksi => {
                    let a = stack.pop().unwrap();
                    stack.push(a.çıkar(Object::Sayı(1.0)));
                    self.current += 1;
                },
                TokenType::Çarpı => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a.çarp(b));
                    self.current += 1;
                },
                TokenType::Bölü => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.böl(a));
                    self.current += 1;
                },
                TokenType::Sayı { val } => {
                    let n = Object::Sayı(val);
                    stack.push(n);
                    self.current += 1;
                },
                TokenType::Yazı { val } => {
                    let s = Object::Yazı(val);
                    stack.push(s);
                    self.current += 1;
                },
                TokenType::Bool { val } => {
                    let b = Object::Bool(val);
                    stack.push(b);
                    self.current += 1;
                },
                TokenType::İse ( yoksa ) | TokenType::İken ( yoksa ) => {
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
                    } else { unreachable!() }
                },
                TokenType::Kopya => {
                    let last = stack.pop().unwrap();
                    stack.push(last.clone());
                    stack.push(last);
                    self.current += 1;
                },
                TokenType::Büyüktür => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.büyüktür(a));
                    self.current += 1;
                },
                TokenType::BüyükEşittir => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.büyük_eşittir(a));
                    self.current += 1;
                },
                TokenType::Küçüktür => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.küçüktür(a));
                    self.current += 1;
                },
                TokenType::KüçükEşittir => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.küçük_eşittir(a));
                    self.current += 1;
                },
                TokenType::Eşittir => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.eşittir(a));
                    self.current += 1;
                },
                TokenType::EşitDeğildir => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.eşit_değildir(a));
                    self.current += 1;
                },
                TokenType::Değildir => {
                    let a = stack.pop().unwrap();
                    stack.push(a.değildir());
                    self.current += 1;
                },
                TokenType::Son { tp } => {
                    self.current = tp;
                },
                TokenType::Yoksa ( yoksa ) => {
                    if let Some(tp) = yoksa {
                        self.current = tp;
                    } else { unreachable!() }
                },
                TokenType::Modulo => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.modulo(a));
                    self.current += 1;
                },
                TokenType::Takas => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a);
                    stack.push(b);
                    self.current += 1;
                },
                TokenType::Döndür => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let c = stack.pop().unwrap();
                    stack.push(a);
                    stack.push(b);
                    stack.push(c);
                    self.current += 1;
                },
                TokenType::At => {
                    stack.pop().unwrap();
                }
                TokenType::Üst => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.clone());
                    stack.push(a);
                    stack.push(b);
                },
                TokenType::Girdi => {
                    let mut buf = String::new();
                    io::stdin().read_to_string(&mut buf).unwrap();
                    stack.push(Object::Yazı(buf.trim_end().to_string()));
                },
                TokenType::İkiNoktaNokta | TokenType::EOF => self.current += 1,
                TokenType::Koy | TokenType::Ve | TokenType::Veya | TokenType::Identifier {..} => unimplemented!(),
            }
        }
    }
}
