use std::fs;
use std::env;

fn char_in_str(a: char, b: &str) -> bool {
    for ch in b.chars() {
        if ch == a {
            return true;
        }
    }
    false
}

struct Lexer {
    source: Vec<char>,
    current:    usize,
    line:       usize,
    col:        usize,
}

#[derive(Debug)]
enum TokenType {
    Yazı,
    Sayı,
    De,
    İken,
    İse,
    Yoksa,
    Identifier,
    Son,
    Artı,
    ArtıArtı,
    Eksi,
    EksiEksi,
    Çarpı,
    Bölü,
    Modulo,
}

#[derive(Debug)]
struct Token {
    typ: TokenType,
    lexeme: String,
    line:    usize,
    col:     usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize, col: usize) -> Self {
        Self { typ, lexeme, line, col }
    }
}

impl Lexer {
    fn new(content: String) -> Self {
        Self {
            source:  content.chars().collect(),
            current: 0,
            line:    1,
            col:     1,
        }
    }
    fn lex(&mut self) -> Vec<Token> {
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
                            match self.currentc() {
                                't' => buf.push('\t'),
                                'n' => buf.push('\n'),
                                'r' => buf.push('\r'),
                                '"' => buf.push('"'),
                                '\'' => buf.push('\''),
                                '\\' => buf.push('\\'),
                                '\n' => (),
                                '\t' => (),
                                _ => {
                                    buf.push('\\');
                                    buf.push(self.currentc())
                                },
                            }
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
                    self.current += 1;
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
                        if self.currentc() == '+' {
                            self.current += 1;
                            self.col     += 1;
                            tokens.push(Token::new(TokenType::EksiEksi, "--".to_string(), self.line, self.col))
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
                ' ' => {
                    self.current += 1;
                    self.col += 1;
                },
                _ => {
                    let mut buf = String::new();

                    while self.source.len() > self.current && self.currentc() != ' ' {
                        buf.push(self.currentc());
                    }

                    match buf.as_str() {
                        "de" => tokens.push(Token::new(TokenType::De, "de".to_string(), self.line, self.col)),
                        "ise" => tokens.push(Token::new(TokenType::İse, "ise".to_string(), self.line, self.col)),
                        "son" => tokens.push(Token::new(TokenType::Son, "son".to_string(), self.line, self.col)),
                        "iken" => tokens.push(Token::new(TokenType::İken, "iken".to_string(), self.line, self.col)),
                        "yoksa" => tokens.push(Token::new(TokenType::Yoksa, "yoksa".to_string(), self.line, self.col)),
                        a => tokens.push(Token::new(TokenType::Identifier, a.to_string(), self.line, self.col))
                    }

                    self.current += 1;
                    self.col += 1;
                },
            }
        }
        tokens
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cont = String::new();
    if args.len() > 1 {
        cont = fs::read_to_string(args.get(1).unwrap()).unwrap();
    }
    let cont = cont;
    let mut lexer = Lexer::new(cont);
    let lexed = lexer.lex();
    println!("{:?}", lexed);
}
