use crate::token::tokentypes::LexerTokenType as TokenType;
use crate::token::LexerToken as Token;
use crate::token::Precedence;
use crate::util::{char_in_str, in_vec, read_file, FSErr};

use std::fs::canonicalize;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            source: content.chars().collect(),
            current: 0,
            line: 1,
            col: 1,
        }
    }
    fn post_proc(&self, prog: Vec<Token>, visited: &mut Vec<String>, file: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut current: usize = 0;

        while current < prog.len() - 1 {
            let c: Token = prog.get(current).unwrap().clone();

            match c.typ {
                TokenType::Yükle => {
                    let next_token = match prog.get(current + 1) {
                        Some(e) => e.clone(),
                        None => panic!("yükle den sonra <yazı> bekleniyordu ancak bulunamadı"),
                    };
                    match next_token.typ {
                        TokenType::Yazı => {
                            let mut tmp_visited = visited.clone();
                            let mut path = canonicalize(PathBuf::from(file.clone()))
                                .expect("yol normalleştirilemedi");
                            if !path.is_dir() {
                                path.set_file_name(next_token.lexeme);
                            } else {
                                path.push(next_token.lexeme);
                            }
                            let source = match read_file(&path) {
                                Ok(f) => {
                                    tmp_visited.push(path.display().to_string());
                                    f
                                },
                                Err(FSErr::IsADir) => {
                                    path.push("giriş.trl");
                                    match read_file(&path) {
                                        Ok(f) => {
                                            tmp_visited.push(path.display().to_string());
                                            f
                                        },
                                        Err(e) => panic!("{:?}", e)
                                    }
                                },
                            };
                            if !in_vec(&path.display().to_string(), &visited.clone()) {
                                let mut nl = Lexer::new(source);
                                let mut res = nl.tokenize(visited, path.display().to_string());

                                let next_token = match prog.get(current + 2) {
                                    Some(e) => e.clone(),
                                    None => panic!("yükle <yazı> dan sonra `*` veya `->` bekleniyordu ancak bulunamadı"),
                                };
                                match next_token.typ {
                                    TokenType::Çarpı => {
                                        tokens.append(&mut res);
                                        current += 3;
                                    }
                                    TokenType::Koy => {
                                        tokens.push(Token::new(
                                            TokenType::Blok,
                                            "blok".to_string(),
                                            next_token.line,
                                            next_token.col,
                                            next_token.file.clone(),
                                            Precedence::Reserved,
                                        ));
                                        let next_token = match prog.get(current + 3) {
                                            Some(e) => e.clone(),
                                            None => panic!("yükle <yazı> -> dan sonra tanımlayıcı bekleniyordu ancak bulunamadı"),
                                        };
                                        match next_token.typ {
                                            TokenType::Identifier => tokens.push(next_token.clone()),
                                            _ => panic!("yükle <yazı> -> dan sonra tanımlayıcı bekleniyordu ancak bulunamadı"),
                                        }
                                        tokens.append(&mut res);
                                        tokens.push(Token::new(
                                            TokenType::Son,
                                            "son".to_string(),
                                            next_token.line,
                                            next_token.col,
                                            next_token.file,
                                            Precedence::Reserved,
                                        ));
                                        current += 4;
                                    }
                                    _ => panic!("yükle <yazı> dan sonra `*` veya `->` bekleniyordu ancak bulunamadı"), 
                                }
                            } else {
                                *visited = tmp_visited;
                            }
                        },
                        _ => panic!("yükle den sonra <yazı> bekleniyordu ancak bulunamadı"),
                    }
                },
                _ => {
                    tokens.push(c);
                    current += 1;
                }
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
                }
                '(' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::ParenL,
                        "(".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::ParenL,
                    ))
                }
                ')' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::ParenR,
                        ")".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::ParenR,
                    ))
                }
                ',' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::Comma,
                        ",".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::Comma,
                    ))
                }
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
                                }
                            }
                            self.col += 1;
                            self.current += 1;
                        }
                    }
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Yazı,
                        buf,
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::None,
                    ))
                }
                b if b.is_numeric() => {
                    let mut buf = String::new();
                    let mut dot_used = false;

                    while self.source.len() > self.current
                        && (self.currentc().is_numeric() || self.currentc() == '.')
                    {
                        if self.currentc() != '.' {
                            buf.push(self.currentc());
                        } else {
                            if dot_used {
                                panic!("Sayılarda birden fazla nokta olamaz");
                            } else {
                                buf.push('.');
                                dot_used = true;
                            }
                        }
                        self.current += 1;
                        self.col += 1;
                    }
                    tokens.push(Token::new(
                        TokenType::Sayı,
                        buf,
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::None,
                    ));
                }
                '\n' => {
                    self.current += 1;
                    self.line += 1;
                    self.col = 1;
                }
                '+' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '+' {
                            self.current += 1;
                            self.col += 1;
                            tokens.push(Token::new(
                                TokenType::ArtıArtı,
                                "++".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::None,
                            ))
                        } else {
                            tokens.push(Token::new(
                                TokenType::Artı,
                                "+".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(2),
                            ))
                        }
                    }
                }
                '-' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '-' {
                            self.current += 1;
                            self.col += 1;
                            tokens.push(Token::new(
                                TokenType::EksiEksi,
                                "--".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::None,
                            ))
                        } else if self.currentc() == '*' {
                            loop {
                                self.current += 1;
                                self.col += 1;
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
                                            self.col += 1;
                                            break;
                                        }
                                    } else {
                                        panic!("unterminated comment");
                                    }
                                }
                            }
                        } else if self.currentc() == '>' {
                            self.current += 1;
                            self.col += 1;
                            tokens.push(Token::new(
                                TokenType::Koy,
                                "->".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Reserved,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Eksi,
                                "-".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(2),
                            ))
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Eksi,
                            "-".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(2),
                        ))
                    }
                }
                '*' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Çarpı,
                        "*".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::Precedence(3),
                    ))
                }
                '/' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Bölü,
                        "/".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::Precedence(3),
                    ))
                }
                '%' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Modulo,
                        "%".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::Precedence(3),
                    ))
                }
                '>' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(
                                TokenType::BüyükEşittir,
                                ">=".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Büyüktür,
                                ">".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Büyüktür,
                            ">".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(1),
                        ));
                    }
                }
                '<' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(
                                TokenType::KüçükEşittir,
                                "<=".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Küçüktür,
                                "<".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Küçüktür,
                            "<".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(1),
                        ));
                    }
                }
                '!' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '=' {
                            tokens.push(Token::new(
                                TokenType::EşitDeğildir,
                                "!=".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Değildir,
                                "!".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(0),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Değildir,
                            "!".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(0),
                        ));
                    }
                }
                '=' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '?' {
                            tokens.push(Token::new(
                                TokenType::Son,
                                "=?".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Eşittir,
                                "=".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Eşittir,
                            "=".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(1),
                        ));
                    }
                }
                ':' => {
                    self.col += 1;
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '.' {
                            tokens.push(Token::new(
                                TokenType::İkiNoktaNokta,
                                ":.".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else if self.currentc() == '?' {
                            tokens.push(Token::new(
                                TokenType::Yoksa,
                                ":?".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::İkiNokta,
                                ":".to_string(),
                                self.line,
                                self.col,
                                file.clone(),
                                Precedence::None,
                            ));
                        }
                    }
                }
                '?' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::İse,
                        "?".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::Reserved,
                    ));
                }
                '@' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::Tipinde,
                        "@".to_string(),
                        self.line,
                        self.col,
                        file.clone(),
                        Precedence::None,
                    ));
                }
                ' ' => {
                    self.current += 1;
                    self.col += 1;
                }
                _ => {
                    let mut buf = String::new();

                    while self.source.len() > self.current
                        && !char_in_str(self.currentc(), "\t\r \n\"':?=<>!/%*@,()")
                    {
                        buf.push(self.currentc());
                        self.current += 1;
                        self.col += 1;
                    }

                    match buf.as_str() {
                        "at" => tokens.push(Token::new(
                            TokenType::At,
                            "at".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "ver" => tokens.push(Token::new(
                            TokenType::Ver,
                            "ver".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "de" => tokens.push(Token::new(
                            TokenType::De,
                            "de".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "ise" => tokens.push(Token::new(
                            TokenType::İse,
                            "ise".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "son" => tokens.push(Token::new(
                            TokenType::Son,
                            "son".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "iken" => tokens.push(Token::new(
                            TokenType::İken,
                            "iken".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "yoksa" => tokens.push(Token::new(
                            TokenType::Yoksa,
                            "yoksa".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "doğru" => tokens.push(Token::new(
                            TokenType::Doğru,
                            "doğru".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "yanlış" => tokens.push(Token::new(
                            TokenType::Yanlış,
                            "yanlış".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "kpy" => tokens.push(Token::new(
                            TokenType::Kopya,
                            "kpy".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "tks" => tokens.push(Token::new(
                            TokenType::Takas,
                            "tks".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "üst" => tokens.push(Token::new(
                            TokenType::Üst,
                            "üst".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "veya" => tokens.push(Token::new(
                            TokenType::Veya,
                            "veya".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(0),
                        )),
                        "ve" => tokens.push(Token::new(
                            TokenType::Ve,
                            "ve".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Precedence(0),
                        )),
                        "dön" => tokens.push(Token::new(
                            TokenType::Döndür,
                            "dön".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "girdi" => tokens.push(Token::new(
                            TokenType::Girdi,
                            "girdi".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "işlev" => tokens.push(Token::new(
                            TokenType::İşlev,
                            "işlev".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "yükle" => tokens.push(Token::new(
                            TokenType::Yükle,
                            "yükle".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        "hiç" => tokens.push(Token::new(
                            TokenType::Hiç,
                            "hiç".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                        "blok" => tokens.push(Token::new(
                            TokenType::Blok,
                            "blok".to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::Reserved,
                        )),
                        a => tokens.push(Token::new(
                            TokenType::Identifier,
                            a.to_string(),
                            self.line,
                            self.col,
                            file.clone(),
                            Precedence::None,
                        )),
                    }
                }
            }
        }
        tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line,
            self.col,
            file.clone(),
            Precedence::Reserved,
        ));
        self.post_proc(tokens, visited, file)
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}
