use crate::token::LexerToken;
use crate::token::tokentypes::LexerTokenType;

#[derive(Debug, Clone)]
pub enum IndentOptions {
    Spaces(u8),
    Tabs,
}
impl Into<String> for IndentOptions {
    fn into(self) -> String {
        match self {
            IndentOptions::Tabs => "\t".to_string(),
            IndentOptions::Spaces(spaces) => {
                let mut s = String::new();
                for _ in 0..spaces {
                    s += " ";
                }
                s
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineEnding {
    LF,
    CRLF,
}
impl AsRef<str> for LineEnding {
    fn as_ref(&self) -> &'static str {
        match self {
            LineEnding::LF => "\n",
            LineEnding::CRLF => "\r\n",
        }
    }
}

#[derive(Debug)]
pub struct Fmt {
    tokens: Vec<LexerToken>,
    _indent: IndentOptions,
    _line_ending: LineEnding,
}
impl Fmt {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            tokens,
            _indent: IndentOptions::Spaces(4),
            _line_ending: LineEnding::LF,
        }
    }
    pub fn indent(self, indent: IndentOptions) -> Self {
        Self {
            _indent: indent,
            ..self
        }
    }
    pub fn line_ending(self, line_ending: LineEnding) -> Self {
        Self {
            _line_ending: line_ending,
            ..self
        }
    }
    pub fn fmt(&self) -> String {
        let mut indent_level: u8 = 0;
        let space: String = self._indent.clone().into();
        let mut result = String::new();
        let mut at_line_start = true;
        let mut current = 0;
        while self.tokens.len() > current {
            let token = self.tokens.get(current).unwrap();
            match token.typ {
                LexerTokenType::İse | LexerTokenType::İken => {
                    let repr = token.repr();
                    at_line_start = true;
                    result += &repr;
                    indent_level += 1;
                },
                LexerTokenType::Yoksa => {
                    let repr = token.repr();
                    if !result.lines().last().unwrap().trim().is_empty() {
                        result += self._line_ending.as_ref();
                        for _ in 1..indent_level {
                            result += &space;
                        }
                    } else {
                        for _ in 0..Into::<String>::into(self._indent.clone()).len() {
                            result += "\x08"; 
                        }
                    }
                    result += &repr;
                    at_line_start = true;
                }
                LexerTokenType::De | LexerTokenType::Ver => {
                    let repr = token.repr();
                    at_line_start = true;
                    result += &repr;
                }
                LexerTokenType::Son => {
                    let repr = token.repr();
                    at_line_start = true;
                    indent_level -= 1;
                    for _ in 0..Into::<String>::into(self._indent.clone()).len() {
                        if result.chars().filter(|c| c != &'\x08').last().unwrap().is_whitespace() {
                            result += "\x08";
                        } else {
                            result += " ";
                        }
                    }
                    result += &repr;
                    if indent_level <= 0 {
                        result += self._line_ending.as_ref();
                    }
                }
                LexerTokenType::İşlev | LexerTokenType::Blok => {
                    if !result.lines().last().unwrap().chars().all(|c| c.is_whitespace()) {
                        result += self._line_ending.as_ref();
                    }
                    let repr = token.repr();
                    let next_tok_repr = self.tokens.get(current + 1).unwrap().repr();
                    result += &repr;
                    result += " ";
                    result += &next_tok_repr;
                    result += " ";
                    current += 1;
                    at_line_start = true;
                    indent_level += 1;
                    while self.tokens.len() - 1 > current {
                        let t = self.tokens.get(current + 1).unwrap();
                        if let LexerTokenType::Koy = t.typ {
                            result += &t.repr();
                            result += " ";
                            let t = self.tokens.get(current + 2).unwrap();
                            result += &t.repr();
                            result += " ";
                            current += 2;
                        } else {
                            break;
                        }
                    }
                }
                LexerTokenType::Koy => {
                    let repr = token.repr();
                    let next_tok_repr = self.tokens.get(current + 1).unwrap().repr();
                    result += &repr;
                    result += " ";
                    result += &next_tok_repr;
                    current += 1;
                    at_line_start = true;
                }
                LexerTokenType::EOF => (),
                LexerTokenType::İkiNokta => {
                    if result.chars().filter(|c| c != &'\x08').last().unwrap().is_whitespace() {
                        result += "\x08";
                    }
                    result += &token.repr()
                }
                LexerTokenType::ParenR | LexerTokenType::InScopeParentR => {
                    if result.chars().filter(|c| c != &'\x08').last().unwrap().is_whitespace() {
                        result += "\x08";
                    }
                    result += &token.repr();
                    let t = self.tokens.get(current + 1);
                    if !matches!(t, Some(LexerToken { typ: LexerTokenType::Identifier | LexerTokenType::Değildir, .. })) {
                        result += " ";
                    }
                }
                LexerTokenType::ParenL | LexerTokenType::InScopeParentL | LexerTokenType::Değildir => result += &token.repr(),
                LexerTokenType::Comma => {
                    if result.chars().filter(|c| c != &'\x08').last().unwrap().is_whitespace() {
                        result += "\x08";
                    }
                    result += &token.repr();
                    result += " ";
                }
                LexerTokenType::Tipinde => {
                    if result.chars().filter(|c| c != &'\x08').last().unwrap().is_whitespace() {
                        result += "\x08";
                    }
                    result += &token.repr();
                }
                LexerTokenType::Yükle => {
                    result += &token.repr();
                    result += " ";
                    current += 1;
                    if self.tokens.len() - 1 > current {
                        let path = self.tokens.get(current).unwrap();
                        result += &path.repr();
                        let act = self.tokens.get(current + 1).unwrap();
                        if let LexerTokenType::Çarpı = act.typ {
                            result += &act.repr();
                        } else if let LexerTokenType::Koy = act.typ {
                            result += " ";
                            result += &act.repr();
                            result += " ";
                            current += 2;
                            if self.tokens.len() > current {
                                let id = self.tokens.get(current).unwrap();
                                result += &id.repr();
                            }
                        }
                    }
                    current += 1;
                    at_line_start = true;
                }
                _ => {
                    result += &token.repr();
                    result += " ";
                }
            }
            if at_line_start {
                result += self._line_ending.as_ref();
                for _ in 0..indent_level {
                    result += &space;
                }
                at_line_start = false;
            }
            current += 1;
        }
        result
    }
}

