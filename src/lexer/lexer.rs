use crate::token::LexerToken as Token;
use crate::token::tokentypes::LexerTokenType as TokenType;
use crate::util::{char_in_str, in_vec, read_file, FSErr};
use crate::store::PATH_SEP;

use std::path::PathBuf;
use std::fs::canonicalize;

#[derive(Clone)]
pub struct Lexer {
    source: Vec<char>,
    current:    usize,
    line:       usize,
    col:        usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            source:  content.chars().collect(),
            current: 0,
            line:    1,
            col:     1,
        }
    }
    fn post_proc(&self, prog: Vec<Token>, visited: &mut Vec<String>, file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current: usize = 0;

        while current < prog.len() - 1 {
            let c: Token = prog.get(current).unwrap().clone();
            
            match c.typ {
                TokenType::Yükle => {
                    let next_tok = prog.get(current + 1).unwrap().clone();
                    match next_tok.typ {
                        TokenType::Yazı => {
                            let pathstr = next_tok.lexeme;
                            let path = PathBuf::from(pathstr.clone());
                            let canon_path: String = if path.has_root() {
                                match canonicalize(&path) {
                                    Ok(a) => a.as_path().display().to_string(),
                                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: `{}`", pathstr, e),
                                }
                            } else {
                                let fold = file.rsplit_once(PATH_SEP);
                                match canonicalize(match fold {
                                    Some((a, _)) => a.to_string(),
                                    None => file.clone(),
                                } + &PATH_SEP.to_string() + &pathstr) {
                                    Ok(a) => a.as_path().display().to_string(),
                                    Err(e) => panic!("`{}` adlı dosya yüklenemedi: {}", pathstr, e),
                                }
                            };

                            let mut path = PathBuf::from(canon_path.clone());
                            if !in_vec(&canon_path, &visited) {
                                visited.push(canon_path.clone());

                                let mut nl = Self::new(match read_file(&path) {
                                    Ok(f) => f,
                                    Err(FSErr::IsADir) => {
                                        path.push("main.trl");
                                        read_file(&path).unwrap()
                                    },
                                });
                                tokens.append(&mut nl.tokenize(visited, canon_path));
                            }
                        },
                        TokenType::Identifier => {},
                        _ => panic!("yükle anahtar kelimesinden sonra yazı veya tanımlayıcı bekleniyordu ancak bulunamadı"), // SyntaxError
                    }
                },
                _ => {
                    tokens.push(c);
                    current += 1;
                },
            }
        }
        tokens
    }
    pub fn tokenize(&mut self, visited: &mut Vec<String>, file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.current < self.source.len() {
            let c: char = self.currentc();

            match c {
                '#' => {
                    while self.current < self.source.len() && self.currentc() != '\n' {
                        self.current += 1;
                        self.col += 1;
                    }
                    self.line += 1;
                },
                '\'' | '"' => {
                    let mut buf = String::new();

                    self.current += 1;
                    while self.currentc() != c {
                        if self.currentc() == '\n' {
                            self.line += 1;
                            self.col = 1;
                        } else {
                            self.col += 1;
                        }
                        if self.currentc() != '\\' {
                            buf.push(self.currentc());
                            self.current += 1;
                        } else {
                            self.current += 1;
                            self.col += 1;
                            match self.currentc() {
                                't' => buf.push('\t'),
                                'n' => buf.push('\n'),
                                'r' => buf.push('\r'),
                                '"' => buf.push('"'),
                                '\'' => buf.push('\''),
                                '\\' => buf.push('\\'),
                                '\n' | '\t' => (),
                                _ => {
                                    buf.push('\\');
                                    buf.push(self.currentc())
                                },
                            }
                            self.col += 1;
                            self.current += 1;
                        }
                    }
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Yazı, buf, self.line, self.col, file.clone()))
                },
                b if b.is_numeric() => {
                    let mut buf = String::new();
                    let mut dot_used = false;

                    while self.source.len() > self.current && (self.currentc().is_numeric() || self.currentc() == '.') {
                        if self.currentc() != '.' {
                            buf.push(self.currentc());
                        } else {
                            if dot_used { panic!("Sayılarda birden fazla nokta olamaz"); }
                            else {
                                buf.push('.');
                                dot_used = true;
                            }
                        }
                        self.current += 1;
                        self.col += 1;
                    }
                    tokens.push(Token::new(TokenType::Sayı, buf, self.line, self.col, file.clone()));
                },
                '\n' => {
                    self.current += 1;
                    self.line += 1;
                    self.col  =  1;
                },
                '+' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '+' {
                            self.current += 1;
                            self.col     += 1;
                            tokens.push(Token::new(TokenType::ArtıArtı, "++".to_string(), self.line, self.col, file.clone()))
                        } else {
                            tokens.push(Token::new(TokenType::Artı, "+".to_string(), self.line, self.col, file.clone()))
                        }
                    }
                },
                '-' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '-' {
                            self.current += 1;
                            self.col     += 1;
                            tokens.push(Token::new(TokenType::EksiEksi, "--".to_string(), self.line, self.col, file.clone()))
                        } else if self.currentc() == '*' {
                            loop {
                                self.current += 1;
                                self.col     += 1;
                                if self.current > self.source.len() {
                                    panic!("unterminated comment");
                                }
                                if self.currentc() == '\n' {
                                    self.line += 1;
                                    self.col = 1;
                                } else if self.currentc() == '*' {
                                    self.current += 1;
                                    self.col += 1;
                                    if self.source.len() > self.current {
                                        if self.currentc() == '-' {
                                            self.current += 1;
                                            self.col     += 1;
                                            break;
                                        }
                                    } else {
                                        panic!("unterminated comment");
                                    }
                                }
                            }
                        } else if self.currentc() == '>' {
                            self.current += 1;
                            self.col     += 1;
                            tokens.push(Token::new(TokenType::Koy, "->".to_string(), self.line, self.col, file.clone()));
                        } else {
                            tokens.push(Token::new(TokenType::Eksi, "-".to_string(), self.line, self.col, file.clone()))
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Eksi, "-".to_string(), self.line, self.col, file.clone()))
                    }
                },
                '*' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Çarpı, "*".to_string(), self.line, self.col, file.clone()))
                },
                '/' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Bölü, "/".to_string(), self.line, self.col, file.clone()))
                },
                '%' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Modulo, "%".to_string(), self.line, self.col, file.clone()))
                },
                '>' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::BüyükEşittir, ">=".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Büyüktür, ">".to_string(), self.line, self.col, file.clone()));
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Büyüktür, ">".to_string(), self.line, self.col, file.clone()));
                    }
                },
                '<' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::KüçükEşittir, "<=".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Küçüktür, "<".to_string(), self.line, self.col, file.clone()));
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Küçüktür, "<".to_string(), self.line, self.col, file.clone()));
                    }
                },
                '!' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::EşitDeğildir, "!=".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Değildir, "!".to_string(), self.line, self.col, file.clone()));
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Değildir, "!".to_string(), self.line, self.col, file.clone()));
                    }
                },
                '=' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '?' {
                            tokens.push(Token::new(TokenType::Son, "=?".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Eşittir, "=".to_string(), self.line, self.col, file.clone()));
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Eşittir, "=".to_string(), self.line, self.col, file.clone()));
                    }
                },
                ':' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '.' {
                            tokens.push(Token::new(TokenType::İkiNoktaNokta, ":.".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else if self.currentc() == '?' {
                            tokens.push(Token::new(TokenType::Yoksa, ":?".to_string(), self.line, self.col, file.clone()));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            unimplemented!("':' operatorü implemente edilmiş değil");
                        }
                    }
                },
                '?' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(TokenType::İse, "?".to_string(), self.line, self.col, file.clone()));
                },
                '@' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(TokenType::Tipinde, "@".to_string(), self.line, self.col, file.clone()));
                },
                ' ' => {
                    self.current += 1;
                    self.col += 1;
                },
                _ => {
                    let mut buf = String::new();

                    while self.source.len() > self.current && !char_in_str(self.currentc(), "\t\r \n\"':?=<>!/%*@") {
                        buf.push(self.currentc());
                        self.current += 1;
                        self.col += 1;
                    }

                    match buf.as_str() {
                        "at" => tokens.push(Token::new(TokenType::At, "at".to_string(), self.line, self.col, file.clone())),
                        "de" => tokens.push(Token::new(TokenType::De, "de".to_string(), self.line, self.col, file.clone())),
                        "ise" => tokens.push(Token::new(TokenType::İse, "ise".to_string(), self.line, self.col, file.clone())),
                        "son" => tokens.push(Token::new(TokenType::Son, "son".to_string(), self.line, self.col, file.clone())),
                        "iken" => tokens.push(Token::new(TokenType::İken, "iken".to_string(), self.line, self.col, file.clone())),
                        "yoksa" => tokens.push(Token::new(TokenType::Yoksa, "yoksa".to_string(), self.line, self.col, file.clone())),
                        "doğru" => tokens.push(Token::new(TokenType::Doğru, "doğru".to_string(), self.line, self.col, file.clone())),
                        "yanlış" => tokens.push(Token::new(TokenType::Yanlış, "yanlış".to_string(), self.line, self.col, file.clone())),
                        "kpy" => tokens.push(Token::new(TokenType::Kopya, "kpy".to_string(), self.line, self.col, file.clone())),
                        "tks" => tokens.push(Token::new(TokenType::Takas, "tks".to_string(), self.line, self.col, file.clone())),
                        "üst" => tokens.push(Token::new(TokenType::Üst, "üst".to_string(), self.line, self.col, file.clone())),
                        "veya" => tokens.push(Token::new(TokenType::Veya, "veya".to_string(), self.line, self.col, file.clone())),
                        "ve" => tokens.push(Token::new(TokenType::Ve, "ve".to_string(), self.line, self.col, file.clone())),
                        "dön" => tokens.push(Token::new(TokenType::Döndür, "dön".to_string(), self.line, self.col, file.clone())),
                        "girdi" => tokens.push(Token::new(TokenType::Girdi, "girdi".to_string(), self.line, self.col, file.clone())),
                        "işlev" => tokens.push(Token::new(TokenType::İşlev, "işlev".to_string(), self.line, self.col, file.clone())),
                        "yükle" => tokens.push(Token::new(TokenType::Yükle, "yükle".to_string(), self.line, self.col, file.clone())),
                        a => tokens.push(Token::new(TokenType::Identifier, a.to_string(), self.line, self.col, file.clone())),
                    }
                },
            }
        }
        tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line, self.col, file.clone()));
        self.post_proc(tokens, visited, file)
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}
