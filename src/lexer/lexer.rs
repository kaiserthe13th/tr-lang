use crate::token::LexerToken as Token;
use crate::token::tokentypes::LexerTokenType as TokenType;
use crate::util::char_in_str;

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
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.current < self.source.len() {
            let c: char = self.currentc();

            match c {
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
                    tokens.push(Token::new(TokenType::Yazı, buf, self.line, self.col))
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
                    tokens.push(Token::new(TokenType::Sayı, buf, self.line, self.col));
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
                            tokens.push(Token::new(TokenType::ArtıArtı, "++".to_string(), self.line, self.col))
                        } else {
                            tokens.push(Token::new(TokenType::Artı, "+".to_string(), self.line, self.col))
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
                            tokens.push(Token::new(TokenType::EksiEksi, "--".to_string(), self.line, self.col))
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
                            tokens.push(Token::new(TokenType::Koy, "->".to_string(), self.line, self.col));
                        } else {
                            tokens.push(Token::new(TokenType::Eksi, "-".to_string(), self.line, self.col))
                        }
                    }
                },
                '*' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Çarpı, "*".to_string(), self.line, self.col))
                },
                '/' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Bölü, "/".to_string(), self.line, self.col))
                },
                '%' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(TokenType::Modulo, "%".to_string(), self.line, self.col))
                },
                '>' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::BüyükEşittir, ">=".to_string(), self.line, self.col));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Büyüktür, ">".to_string(), self.line, self.col));
                        }
                    }
                },
                '<' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::KüçükEşittir, "<=".to_string(), self.line, self.col));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Küçüktür, "<".to_string(), self.line, self.col));
                        }
                    }
                },
                '!' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(TokenType::EşitDeğildir, "!=".to_string(), self.line, self.col));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Değildir, "!".to_string(), self.line, self.col));
                        }
                    }
                },
                '=' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '?' {
                            tokens.push(Token::new(TokenType::Son, "=?".to_string(), self.line, self.col));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(TokenType::Eşittir, "=".to_string(), self.line, self.col));
                        }
                    }
                },
                ':' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '.' {
                            tokens.push(Token::new(TokenType::İkiNoktaNokta, ":.".to_string(), self.line, self.col));
                            self.col += 1;
                            self.current += 1;
                        } else if self.currentc() == '?' {
                            tokens.push(Token::new(TokenType::Yoksa, ":?".to_string(), self.line, self.col));
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
                    tokens.push(Token::new(TokenType::İse, "?".to_string(), self.line, self.col));
                },
                ' ' => {
                    self.current += 1;
                    self.col += 1;
                },
                _ => {
                    let mut buf = String::new();

                    while self.source.len() > self.current && !char_in_str(self.currentc(), "\t\r \n\"':?=<>!/%+-*") {
                        buf.push(self.currentc());
                        self.current += 1;
                        self.col += 1;
                    }

                    match buf.as_str() {
                        "de" => tokens.push(Token::new(TokenType::De, "de".to_string(), self.line, self.col)),
                        "ise" => tokens.push(Token::new(TokenType::İse, "ise".to_string(), self.line, self.col)),
                        "son" => tokens.push(Token::new(TokenType::Son, "son".to_string(), self.line, self.col)),
                        "iken" => tokens.push(Token::new(TokenType::İken, "iken".to_string(), self.line, self.col)),
                        "yoksa" => tokens.push(Token::new(TokenType::Yoksa, "yoksa".to_string(), self.line, self.col)),
                        "doğru" => tokens.push(Token::new(TokenType::Doğru, "doğru".to_string(), self.line, self.col)),
                        "yanlış" => tokens.push(Token::new(TokenType::Yanlış, "yanlış".to_string(), self.line, self.col)),
                        "kpy" => tokens.push(Token::new(TokenType::Kopya, "kpy".to_string(), self.line, self.col)),
                        "tks" => tokens.push(Token::new(TokenType::Takas, "tks".to_string(), self.line, self.col)),
                        "üst" => tokens.push(Token::new(TokenType::Üst, "üst".to_string(), self.line, self.col)),
                        "veya" => tokens.push(Token::new(TokenType::Veya, "veya".to_string(), self.line, self.col)),
                        "ve" => tokens.push(Token::new(TokenType::Ve, "ve".to_string(), self.line, self.col)),
                        "dön" => tokens.push(Token::new(TokenType::Döndür, "dön".to_string(), self.line, self.col)),
                        "girdi" => tokens.push(Token::new(TokenType::Girdi, "girdi".to_string(), self.line, self.col)),
                        a => tokens.push(Token::new(TokenType::Identifier, a.to_string(), self.line, self.col)),
                    }
                },
            }
        }
        tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line, self.col));
        tokens
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}
