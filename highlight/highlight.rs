use crate::token::tokentypes::LexerTokenType as TokenType;
use crate::token::LexerToken as Token;
use crate::util::{char_in_str, in_vec;

const HTML_TEMPLATE_START: &str = "\
<doctype! html>
<html>
    <head>
        <title>tr-lang</title>
        <link rel=\"stylesheet\" href=\"trl-highlight.css\">
    </head>
    <body>
    <div class=\"cb-header\">
        <div>tr-lang</div>
        <div>
            <button class=\"nobg\" onclick=\"copyInnerText(this.parentElement.parentElement.children[1])\">Copy</button>
        </div>
    </div>
    </pre class=\"code\">";

const HTML_TEMPLATE_END: &str = "</pre><script src=\"trl-highlight.js\"></script></body>
</html>";

struct SpanClasses {
    line_comment: String,
    paren_left: String,
    paren_right: String,
    comma: String,
    single_quoted_string: String,
    double_quoted_string: String,
    string_escape: String,
    number_int: String,
    number_float: String,
    plus: String,
    plus_plus: String,
    minus: String,
    minus_minus: String,
    block_comment: String,
    assign_op: String,
    mul_sign: String,
    div_sign: String,
    modulo: String,
    greater: String,
    greater_eq: String,
    lesser: String,
    lesser_eq: String,
    not_equal: String,
    not: String,
    equals: String,
    question_mark_end: String,
    question_mark_else: String,
    question_mark_if: String,
    question_mark: String,
    double_dot_dot: String,
    convert_to: String,
    identifier: String,
    key_at: String,
    key_ver: String,
    key_de: String,
    key_ise: String,
    key_son: String,
    key_iken: String,
    key_yoksa: String,
    key_doğru: String,
    key_yanlış: String,
    key_kpy: String,
    key_tks: String,
    key_üst: String,
    key_veya: String,
    key_ve: String,
    key_dön: String,
    key_girdi: String,
    key_işlev: String,
    key_yükle: String,
}

pub enum HighlighterType {
    HTMLSpanTag(SpanClasses),
    ANSIEscapeSequence,
}

#[derive(Clone)]
pub struct Highlighter {
    source: Vec<char>,
    current: usize,
    typ: HighlighterType,
    use_template: bool,
}

impl Highlighter {
    pub fn escape_char(c: char) -> &str {
        match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&#x27;",
            '/' => "&x#2F;",
            c => &format!("{}", c),
        }
    }
    pub fn new(content: String, typ: HighlighterType) -> Self {
        Self {
            source: content.chars().collect(),
            current: 0,
            typ
        }
    }
    pub fn highlight(&mut self) -> String {
        let mut result: String::from(if self.use_template {HTML_TEMPLATE_START} else {""});

        while self.current < self.source.len() {
            let c: char = self.currentc();

            match c {
                '#' => {
                    match self.typ {
                        HighlighterType::HTMLSpanTag(sc) => result.push_str(&format!("<span class=\"{}\">", sc.line_comment)),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                    while self.current < self.source.len() && self.currentc() != '\n' {
                        result.push(self.currentc());
                        self.current += 1;
                    }
                    match self.typ {
                        HighlighterType::HTMLSpanTag(_) => result.push_str("</span>"),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                }
                '(' => {
                    self.current += 1;
                    match self.typ {
                        HighlighterType::HTMLSpanTag(sc) => result.push_str("<span class=\"{}\">(</span>", sc.paren_left),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                }
                ')' => {
                    self.current += 1;
                    match self.typ {
                        HighlighterType::HTMLSpanTag(sc) => result.push_str("<span class=\"{}\">)</span>", sc.paren_right),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                }
                ',' => {
                    self.current += 1;
                    match self.typ {
                        HighlighterType::HTMLSpanTag(sc) => result.push_str("<span class=\"{}\">,</span>", sc.comma),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                }
                '\'' | '"' => {
                    match self.typ {
                        HighlighterType::HTMLSpanTag(sc) => result.push_str("<span class=\"{}\">", match c {
                            '\'' => sc.single_quoted_string,
                            '"' => sc.double_quoted_string,
                            _ => unreacheable!(),
                        }),
                        _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                    }
                    result.push_str(self.escape_char(c));
                    self.current += 1;
                    while self.currentc() != c {
                        if self.currentc() != '\\' {
                            result.push(self.escape_char(self.currentc()));
                            self.current += 1;
                        } else {
                            self.current += 1;
                            match self.currentc() {
                                't' | 'n' | 'r' | '"' | '\'' | '\\' | '\n' | '\t' => {
                                    match self.typ {
                                        HTMLSpanTag(sc) => result.push_str(&format!("<span class=\"{}\">\\{}</span>", sc.string_escape, self.escape_char(self.currentc()))),
                                    _ => unimplemented!("tr-lang highlighter only supports HTMLSpanTag mode for now"),
                                },
                                _ => {
                                    buf.push("\\");
                                    buf.push(self.escape_char(self.currentc()));
                                }
                            }
                            self.current += 1;
                        }
                    }
                    self.current += 1;
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
                    self.col = 1;
                }
                '+' => {
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '+' {
                            self.current += 1;
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
                    self.current += 1;
                    if self.source.len() > self.current {
                        if self.currentc() == '-' {
                            self.current += 1;
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
                                if self.current > self.source.len() {
                                    panic!("unterminated comment");
                                }
                                if self.currentc() == '\n' {
                                    self.col = 1;
                                } else if self.currentc() == '*' {
                                    self.current += 1;
                                    if self.source.len() > self.current {
                                        if self.currentc() == '-' {
                                            self.current += 1;
                                            break;
                                        }
                                    } else {
                                        panic!("unterminated comment");
                                    }
                                }
                            }
                        } else if self.currentc() == '>' {
                            self.current += 1;
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
                            self.current += 1;
                        } else {
                            unimplemented!("':' operatorü implemente edilmiş değil");
                        }
                    }
                }
                '?' => {
                    self.current += 1;
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
                }
                _ => {
                    let mut buf = String::new();

                    while self.source.len() > self.current
                        && !char_in_str(self.currentc(), "\t\r \n\"':?=<>!/%*@")
                    {
                        buf.push(self.currentc());
                        self.current += 1;
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
    }
    fn currentc(&self) -> char {
        *self.source.get(self.current).unwrap()
    }
}
