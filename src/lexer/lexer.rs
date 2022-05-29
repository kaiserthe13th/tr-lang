use crate::token::tokentypes::LexerTokenType as TokenType;
use crate::token::LexerToken as Token;
use crate::token::Precedence;
use crate::util::SupportedLanguage;
use crate::util::get_lang;
use crate::util::{char_in_str, in_vec, read_file, FSErr};
use crate::error::Error;

use std::fs::canonicalize;
use std::path::PathBuf;

/// Lexer of tr-lang, made primarily for tokenization
/// but it also manages imports and includes at post_proc
#[derive(Clone)]
pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    col: usize,
    do_post_proc: bool,
}

impl Lexer {
    /// Creates a new lexer from content(source code)
    pub fn new(content: String) -> Self {
        Self {
            source: content.chars().collect(),
            current: 0,
            line: 1,
            col: 1,
            do_post_proc: true,
        }
    }
    pub fn do_post_proc(&mut self, do_post_proc: bool) -> Self {
        self.do_post_proc = do_post_proc;
        Self {
            do_post_proc,
            ..self.clone()
        }
    }
    /// Lexer post processor
    fn post_proc(&self, prog: Vec<Token>, visited: &mut Vec<String>, file: String) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![];
        let mut current: usize = 0;

        while current < prog.len() - 1 {
            let c: Token = prog.get(current).unwrap().clone();

            match c.typ {
                TokenType::Yükle => {
                    let next_token = match prog.get(current + 1) {
                        Some(e) => e.clone(),
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => Error::new(
                                "BeklenmedikSimge",
                                "yükle den sonra <yazı> ya da '(' bekleniyordu ancak bulunamadı",
                                vec![(0, 0, file, None)],
                                None
                            ),
                            SupportedLanguage::English => Error::new(
                                "BeklenmedikSimge",
                                "expected <string> or `(` after `yükle` but couldn't find it",
                                vec![(0, 0, file, None)],
                                None
                            ),
                        })
                    };
                    match next_token.typ {
                        TokenType::Yazı => {
                            let mut tmp_visited = visited.clone();
                            let mut path = canonicalize(PathBuf::from(file.clone()))
                                .expect(match get_lang() {
                                    SupportedLanguage::Turkish => "yol normalleştirilemedi",
                                    SupportedLanguage::English => "couldn't canonicalize path",
                                });
                            if path.is_dir() {
                                path.push(next_token.lexeme);
                            } else {
                                path.set_file_name(next_token.lexeme);
                            }
                            let mut so_lookup = false;
                            let source = match read_file(&path) {
                                Ok(f) => {
                                    tmp_visited.push(path.display().to_string());
                                    f
                                }
                                Err(FSErr::IsADir) => {
                                    path.push("giriş.trl");
                                    match read_file(&path) {
                                        Ok(f) => {
                                            tmp_visited.push(path.display().to_string());
                                            f
                                        }
                                        Err(e) => return Err(match get_lang() {    
                                            SupportedLanguage::Turkish => Error::new(
                                                "DosyaHatası",
                                                &format!("{:?}", e),
                                                vec![(0, 0, path.display().to_string(), None)],
                                                None
                                            ),
                                            SupportedLanguage::English => Error::new(
                                                "DosyaHatası",
                                                &format!("{:?}", e),
                                                vec![(0, 0, path.display().to_string(), None)],
                                                None,
                                            ),
                                        }),
                                    }
                                },
                                Err(FSErr::Other(std::io::ErrorKind::InvalidData)) => {
                                    so_lookup = true;
                                    "".to_string()
                                },
                                Err(e) => return Err(match get_lang() {    
                                    SupportedLanguage::Turkish => Error::new(
                                        "DosyaHatası",
                                        &format!("{:?}", e),
                                        vec![(0, 0, path.display().to_string(), None)],
                                        None
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "DosyaHatası",
                                        &format!("{:?}", e),
                                        vec![(0, 0, path.display().to_string(), None)],
                                        None,
                                    ),
                                }),
                            };
                            if !in_vec(&path.display().to_string(), &visited.clone()) {
                                let res = if !so_lookup {
                                    let mut nl = Lexer::new(source);
                                    nl.tokenize(visited, path.display().to_string())
                                } else {
                                    Ok(vec![Token::new(
                                        TokenType::LibSymbol(path.display().to_string()),
                                        "".to_string(),
                                        0,
                                        0,
                                        file.clone(),
                                        Precedence::Reserved,
                                    )])
                                };

                                let next_token = match prog.get(current + 2) {
                                    Some(e) => e.clone(),
                                    None => return Err(match get_lang() {
                                        SupportedLanguage::Turkish => Error::new(
                                            "BeklenmedikSimge",
                                            "yükle <yazı> dan sonra `*` veya `->` bekleniyordu ancak bulunamadı",
                                            vec![(0, 0, file, None)],
                                            None,
                                        ),
                                        SupportedLanguage::English => Error::new(
                                            "BeklenmedikSimge",
                                            "expected `*` or `->` after `yükle <string>` but couldn't find any",
                                            vec![(0, 0, file, None)],
                                            None,
                                        ),
                                    }),
                                };
                                match next_token.typ {
                                    TokenType::Çarpı => {
                                        tokens.append(&mut res?);
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
                                            None => return Err(match get_lang() {
                                                SupportedLanguage::Turkish => Error::new(
                                                    "BeklenmedikSimge",
                                                    "`yükle <yazı> ->` dan sonra tanımlayıcı bekleniyordu ancak bulunamadı",
                                                    vec![(0, 0, file, None)],
                                                    None,
                                                ),
                                                SupportedLanguage::English => Error::new(
                                                    "BeklenmedikSimge",
                                                    "expected identifier after `yükle <string> ->` but couldn't find it",
                                                    vec![(0, 0, file, None)],
                                                    None,
                                                ),
                                            }),
                                        };
                                        match next_token.typ {
                                            TokenType::Identifier => tokens.push(next_token.clone()),
                                            _ => return Err(match get_lang() {
                                                SupportedLanguage::Turkish => Error::new(
                                                    "BeklenmedikSimge",
                                                    "`yükle <yazı> ->` dan sonra tanımlayıcı bekleniyordu ancak bulunamadı",
                                                    vec![(0, 0, file, None)],
                                                    None,
                                                ),
                                                SupportedLanguage::English => Error::new(
                                                    "BeklenmedikSimge",
                                                    "expected identifier after `yükle <string> ->` but couldn't find it",
                                                    vec![(0, 0, file, None)],
                                                    None,
                                                ),
                                            }),
                                        }
                                        tokens.append(&mut res?);
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
                                    _ => return Err(match get_lang() {
                                        SupportedLanguage::Turkish => Error::new(
                                            "BeklenmedikSimge",
                                            "`yükle <yazı>` dan sonra `*` veya `->` bekleniyordu ancak bulunamadı",
                                            vec![(next_token.line, next_token.col, next_token.file, None)],
                                            None,
                                        ),
                                        SupportedLanguage::English => Error::new(
                                            "BeklenmedikSimge",
                                            "expected `*` or `->` after `yükle <string>` but couldn't find any",
                                            vec![(next_token.line, next_token.col, next_token.file, None)],
                                            None,
                                        ),
                                    }),
                                }
                            } else {
                                *visited = tmp_visited;
                            }
                        }
                        TokenType::ParenL => {
                            tokens.push(Token::new(
                                TokenType::InScopeParentL,
                                next_token.lexeme,
                                next_token.line,
                                next_token.col,
                                next_token.file,
                                Precedence::Reserved,
                            ));
                            current += 2;
                            while current < prog.len() - 1 {
                                let c = prog.get(current).unwrap().clone();
                                current += 1;
                                match c.typ {
                                    TokenType::ParenR => {
                                        tokens.push(Token::new(
                                            TokenType::InScopeParentR,
                                            c.lexeme,
                                            c.line,
                                            c.col,
                                            c.file,
                                            Precedence::Reserved,
                                        ));
                                        break;
                                    }
                                    TokenType::Identifier | TokenType::Çarpı | TokenType::İkiNokta => tokens.push(c),
                                    _ => return Err(match get_lang() {
                                        SupportedLanguage::Turkish => Error::new(
                                            "BeklenmedikSimge",
                                            &format!(
                                                "yükle ('dan sonra tanımlayıcı, `)` veya `*` bekleniyordu ancak bulunamadı. {:?}",
                                                c
                                            ),
                                            vec![(c.line, c.col, c.file, None)],
                                            None
                                        ),
                                        SupportedLanguage::English => Error::new(
                                            "BeklenmedikSimge",
                                            &format!(
                                                "expected identifier, `)` or `*` after `yükle (` but found {:?}",
                                                c
                                            ),
                                            vec![(c.line, c.col, c.file, None)],
                                            None
                                        ),
                                    })
                                }
                            }
                        }
                        _ => return Err(match get_lang() {
                            SupportedLanguage::Turkish => Error::new(
                                "BeklenmedikSimge",
                                "`yükle` den sonra `<yazı>` veya `(` bekleniyordu ancak bulunamadı",
                                vec![(0, 0, file, None)], None
                            ),
                            SupportedLanguage::English => Error::new(
                                "BeklenmedikSimge",
                                "expected `<string>` or `(` after `yükle` but couldn't find any",
                                vec![(0, 0, file, None)], None
                            ),
                        }),
                    }
                }
                _ => {
                    tokens.push(c);
                    current += 1;
                }
            }
        }
        Ok(tokens)
    }
    /// The tokenizer; it will try to tokenize the source code,
    ///
    /// If it encounters errors it will send `Err(tr_lang::error::Error)` back
    pub fn tokenize(&mut self, visited: &mut Vec<String>, file: String) -> Result<Vec<Token>, Error> {
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
                        self.line, self.col, file.clone(),
                        Precedence::ParenL,
                    ))
                }
                ')' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::ParenR,
                        ")".to_string(),
                        self.line, self.col, file.clone(),
                        Precedence::ParenR,
                    ))
                }
                ',' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::Comma,
                        ",".to_string(),
                        self.line, self.col, file.clone(),
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
                        self.line, self.col, file.clone(),
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
                                return Err(match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "SözdizimHatası",
                                        "Sayılarda birden fazla nokta olamaz",
                                        vec![(self.line, self.col, file, None)], None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "SözdizimHatası",
                                        "Numbers can't have more than one dot int them",
                                        vec![(self.line, self.col, file, None)], None,
                                    ),
                                });
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
                        self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::None,
                            ))
                        } else {
                            tokens.push(Token::new(
                                TokenType::Artı,
                                "+".to_string(),
                                self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::None,
                            ))
                        } else if self.currentc() == '*' {
                            loop {
                                self.current += 1;
                                self.col += 1;
                                if self.current > self.source.len() {
                                    return Err(match get_lang() {
                                        SupportedLanguage::English => Error::new(
                                            "SözdizimHatası",
                                            "unterminated comment",
                                            vec![(self.line, self.col, file, None)], None,
                                        ),
                                        SupportedLanguage::Turkish => Error::new(
                                            "SözdizimHatası",
                                            "bitirilmemiş yorum",
                                            vec![(self.line, self.col, file, None)], None,
                                        ),
                                    });
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
                                        return Err(match get_lang() {
                                            SupportedLanguage::English => Error::new(
                                                "SözdizimHatası",
                                                "unterminated comment",
                                                vec![(self.line, self.col, file, None)], None,
                                            ),
                                            SupportedLanguage::Turkish => Error::new(
                                                "SözdizimHatası",
                                                "bitirilmemiş yorum",
                                                vec![(self.line, self.col, file, None)], None,
                                            ),
                                        });
                                    }
                                }
                            }
                        } else if self.currentc() == '>' {
                            self.current += 1;
                            self.col += 1;
                            tokens.push(Token::new(
                                TokenType::Koy,
                                "->".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Reserved,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Eksi,
                                "-".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(2),
                            ))
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Eksi,
                            "-".to_string(),
                            self.line, self.col, file.clone(),
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
                        self.line, self.col, file.clone(),
                        Precedence::Precedence(3),
                    ))
                }
                '/' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Bölü,
                        "/".to_string(),
                        self.line, self.col, file.clone(),
                        Precedence::Precedence(3),
                    ))
                }
                '%' => {
                    self.col += 1;
                    self.current += 1;
                    tokens.push(Token::new(
                        TokenType::Modulo,
                        "%".to_string(),
                        self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Büyüktür,
                                ">".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Büyüktür,
                            ">".to_string(),
                            self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Küçüktür,
                                "<".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Küçüktür,
                            "<".to_string(),
                            self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Değildir,
                                "!".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(0),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Değildir,
                            "!".to_string(),
                            self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::Eşittir,
                                "=".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Precedence(1),
                            ));
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Eşittir,
                            "=".to_string(),
                            self.line, self.col, file.clone(),
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
                                self.line, self.col, file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else if self.currentc() == '?' {
                            tokens.push(Token::new(
                                TokenType::Yoksa,
                                ":?".to_string(),
                                self.line, self.col, file.clone(),
                                Precedence::Reserved,
                            ));
                            self.col += 1;
                            self.current += 1;
                        } else {
                            tokens.push(Token::new(
                                TokenType::İkiNokta,
                                ":".to_string(),
                                self.line, self.col, file.clone(),
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
                        self.line, self.col, file.clone(),
                        Precedence::Reserved,
                    ));
                }
                '@' => {
                    self.current += 1;
                    self.col += 1;
                    tokens.push(Token::new(
                        TokenType::Tipinde,
                        "@".to_string(),
                        self.line, self.col, file.clone(),
                        Precedence::None,
                    ));
                }
                '\r' | ' ' | '\t' => {
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
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "ver" => tokens.push(Token::new(
                            TokenType::Ver,
                            "ver".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "de" => tokens.push(Token::new(
                            TokenType::De,
                            "de".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "ise" => tokens.push(Token::new(
                            TokenType::İse,
                            "ise".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "son" => tokens.push(Token::new(
                            TokenType::Son,
                            "son".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "iken" => tokens.push(Token::new(
                            TokenType::İken,
                            "iken".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "yoksa" => tokens.push(Token::new(
                            TokenType::Yoksa,
                            "yoksa".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "doğru" => tokens.push(Token::new(
                            TokenType::Doğru,
                            "doğru".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "yanlış" => tokens.push(Token::new(
                            TokenType::Yanlış,
                            "yanlış".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "kpy" => tokens.push(Token::new(
                            TokenType::Kopya,
                            "kpy".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "sına" => tokens.push(Token::new(
                            TokenType::Sına,
                            "sına".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "tks" => tokens.push(Token::new(
                            TokenType::Takas,
                            "tks".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "üst" => tokens.push(Token::new(
                            TokenType::Üst,
                            "üst".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "veya" => tokens.push(Token::new(
                            TokenType::Veya,
                            "veya".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Precedence(0),
                        )),
                        "ve" => tokens.push(Token::new(
                            TokenType::Ve,
                            "ve".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Precedence(0),
                        )),
                        "dön" => tokens.push(Token::new(
                            TokenType::Döndür,
                            "dön".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "girdi" => tokens.push(Token::new(
                            TokenType::Girdi,
                            "girdi".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "işlev" => tokens.push(Token::new(
                            TokenType::İşlev,
                            "işlev".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "yükle" => tokens.push(Token::new(
                            TokenType::Yükle,
                            "yükle".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        "hiç" => tokens.push(Token::new(
                            TokenType::Hiç,
                            "hiç".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                        "blok" => tokens.push(Token::new(
                            TokenType::Blok,
                            "blok".to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::Reserved,
                        )),
                        a => tokens.push(Token::new(
                            TokenType::Identifier,
                            a.to_string(),
                            self.line, self.col, file.clone(),
                            Precedence::None,
                        )),
                    }
                }
            }
        }
        tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line, self.col, file.clone(),
            Precedence::Reserved,
        ));
        if self.do_post_proc {
            self.post_proc(tokens, visited, file)
        } else {
            Ok(tokens)
        }
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}
