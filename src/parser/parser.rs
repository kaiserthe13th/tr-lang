use crate::errwarn::{Error, ErrorGenerator};
use crate::lexer;

use crate::token::tokentypes::LexerTokenType as LexTokenType;
use crate::token::LexerToken as LexToken;

use crate::token::tokentypes::ParserTokenType as TokenType;
use crate::token::ParserToken as Token;
use crate::util::{get_lang, SupportedLanguage};
use std::fmt;

#[derive(Clone)]
enum BlockToken {
    İse(usize),
    İken(usize),
    İkiNoktaNokta(usize),
    İşlev(usize),
    Blok(usize),
}

impl fmt::Debug for BlockToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::İse(_) => write!(f, "ise"),
            Self::İken(_) => write!(f, "iken"),
            Self::İkiNoktaNokta(_) => write!(f, ":."),
            Self::İşlev(_) => write!(f, "işlev"),
            Self::Blok(_) => write!(f, "blok"),
        }
    }
}

impl BlockToken {
    fn unwrap_inner(&self) -> usize {
        match self {
            Self::İse(u) | Self::İken(u) | Self::İkiNoktaNokta(u)
                | Self::İşlev(u) | Self::Blok(u) => *u,
        }
    }
}

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<LexToken>,
}

impl Parser {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self {
            tokens: Self::preproc(tokens),
        }
    }

    pub fn from_lexer(lexer: &mut lexer::Lexer, file: String) -> Result<Self, Error> {
        Ok(Self {
            tokens: Self::preproc(lexer.tokenize(&mut vec![], file)?),
        })
    }

    fn preproc(prog: Vec<LexToken>) -> Vec<LexToken> {
        use crate::token::Precedence;

        let mut stack: Vec<LexToken> = vec![];
        let mut current = 0;
        let mut tokens = vec![];

        while current < prog.len() {
            let i = prog.get(current).unwrap();
            match i.precedence {
                Precedence::None => {
                    tokens.push(i.clone());
                    current += 1;
                }
                Precedence::Reserved => {
                    while !stack.is_empty() {
                        tokens.push(stack.pop().unwrap());
                    }
                    tokens.push(i.clone());
                    current += 1;
                }
                Precedence::Precedence(u) => {
                    while !stack.is_empty()
                        && match stack.last().unwrap().precedence {
                            Precedence::Precedence(x) => x > u,
                            Precedence::ParenL => false,
                            _ => unreachable!(),
                        }
                    {
                        tokens.push(stack.pop().unwrap());
                    }
                    stack.push(i.clone());
                    current += 1;
                }
                Precedence::ParenL => {
                    stack.push(i.clone());
                    current += 1;
                }
                Precedence::ParenR => {
                    while !stack.is_empty()
                        && match stack.last().unwrap().precedence {
                            Precedence::ParenL => false,
                            _ => true,
                        }
                    {
                        tokens.push(stack.pop().unwrap());
                    }
                    if !stack.is_empty()
                        && matches!(stack.last().unwrap().precedence, Precedence::ParenL)
                    {
                        stack.pop().unwrap();
                    }
                    current += 1;
                }
                Precedence::Comma => {
                    while !stack.is_empty()
                        && match stack.last().unwrap().precedence {
                            Precedence::ParenL => false,
                            _ => true,
                        }
                    {
                        tokens.push(stack.pop().unwrap());
                    }
                    current += 1;
                }
            }
        }
        while !stack.is_empty() {
            tokens.push(stack.pop().unwrap());
        }
        tokens
    }

    /// Parser for tr-lang. It runs `Parser::preproc` beforehand to reorder operations in a way the
    /// language can understand. After that it will try to parse the tokens and run them. On
    /// failure it will return `Err(tr_lang::errwarn::Error)`
    pub fn parse(&mut self) -> Result<Vec<Token>, Error> {
        let mut parsed: Vec<Token> = vec![];
        let mut blocktokens: Vec<BlockToken> = vec![];
        let mut rets: Vec<Vec<usize>> = vec![vec![]];

        for (ip, ptoken) in self.tokens.iter().enumerate() {
            match ptoken.typ {
                LexTokenType::InScopeParentR => parsed.push(Token::new(
                    TokenType::InScopeParentR,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::InScopeParentL => parsed.push(Token::new(
                    TokenType::InScopeParentL,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::İkiNokta => parsed.push(Token::new(
                    TokenType::İkiNokta,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Blok => {
                    parsed.push(Token::new(
                        TokenType::Blok,
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                    blocktokens.push(BlockToken::Blok(ip));
                }
                LexTokenType::Hiç => parsed.push(Token::new(
                    TokenType::Hiç,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Ver => {
                    parsed.push(Token::new(
                        TokenType::Ver { tp: None },
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                    if let Some(last) = rets.last_mut() {
                        last.push(ip);
                    } else { unreachable!() }
                }
                LexTokenType::Yükle | LexTokenType::Comma => unreachable!(),
                LexTokenType::ParenR | LexTokenType::ParenL => {
                    return Err(match get_lang() {
                        SupportedLanguage::Turkish => ErrorGenerator::error(
                            "SözdizimHatası",
                            "kapatılmamış parantez",
                            ptoken.line, ptoken.col, ptoken.file.clone(),
                            None
                        ),
                        SupportedLanguage::English => ErrorGenerator::error(
                            "SyntaxError",
                            "unclosed parenthesis",
                            ptoken.line, ptoken.col, ptoken.file.clone(),
                            None
                        ),
                    });
                }
                LexTokenType::İşlev => {
                    blocktokens.push(BlockToken::İşlev(ip));
                    parsed.push(Token::new(
                        TokenType::İşlev { sonloc: None },
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                    rets.push(vec![]);
                }
                LexTokenType::At => {
                    parsed.push(Token::new(
                        TokenType::At,
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                }
                LexTokenType::Sayı => {
                    parsed.push(Token::new(
                        TokenType::Sayı {
                            val: ptoken.lexeme.as_str().parse().unwrap(),
                        },
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                }
                LexTokenType::Yazı => parsed.push(Token::new(
                    TokenType::Yazı {
                        val: ptoken.lexeme.clone(),
                    },
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Identifier => parsed.push(Token::new(
                    TokenType::Identifier {
                        id: ptoken.lexeme.clone(),
                    },
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::İkiNoktaNokta => {
                    blocktokens.push(BlockToken::İkiNoktaNokta(ip));
                    parsed.push(Token::new(
                        TokenType::İkiNoktaNokta,
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ))
                }
                LexTokenType::İken => {
                    let last_blocktoken = blocktokens.last().unwrap();
                    match last_blocktoken {
                        BlockToken::İkiNoktaNokta(_) => (),
                        a => {
                            let o = parsed.get(a.unwrap_inner()).unwrap().clone();
                            return Err(match get_lang() {
                                SupportedLanguage::Turkish => ErrorGenerator::error(
                                    "SözdizimHatası",
                                    &format!(
                                        "kapatılmamış blok {:?}",
                                        a
                                    ),
                                    o.line, o.col, o.file,
                                    None,
                                ),
                                SupportedLanguage::English => ErrorGenerator::error(
                                    "SyntaxError",
                                    &format!(
                                        "unclosed block {:?}",
                                        a
                                    ),
                                    o.line, o.col, o.file,
                                    None,
                                ),
                            });
                        },
                    };
                    blocktokens.push(BlockToken::İken(ip));
                    parsed.push(Token::new(
                        TokenType::İken(None),
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                }
                LexTokenType::İse => {
                    blocktokens.push(BlockToken::İse(ip));
                    parsed.push(Token::new(
                        TokenType::İse(None),
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                }
                LexTokenType::Yoksa => {
                    let last_blocktoken = blocktokens.pop().unwrap();
                    match last_blocktoken {
                        BlockToken::İse(bip) => {
                            let ise = &mut parsed[bip];
                            match ise.typ {
                                TokenType::İse(ref mut yoksa) => {
                                    yoksa.replace(ip + 1);
                                }
                                _ => unreachable!(),
                            }
                            ip + 1
                        }
                        _ => unimplemented!(),
                    };
                    blocktokens.push(BlockToken::İse(ip));
                    parsed.push(Token::new(
                        TokenType::Yoksa(None),
                        ptoken.line,
                        ptoken.col,
                        ptoken.file.clone(),
                    ));
                }
                LexTokenType::Son => {
                    let last_blocktoken = blocktokens.pop().unwrap();
                    match last_blocktoken {
                        BlockToken::İse(bip) => {
                            let ise = &mut parsed[bip];
                            match ise.typ {
                                TokenType::İse(ref mut yoksa) => {
                                    yoksa.replace(ip);
                                }
                                TokenType::Yoksa(ref mut tp) => {
                                    tp.replace(ip);
                                }
                                _ => unreachable!(),
                            }
                            let tp = ip + 1;
                            parsed.push(Token::new(
                                TokenType::Son { tp },
                                ptoken.line,
                                ptoken.col,
                                ptoken.file.clone(),
                            ));
                        }
                        BlockToken::İken(bip) => {
                            let iken = parsed.get_mut(bip).unwrap();
                            let tp: usize = match iken.typ {
                                TokenType::İken(ref mut tp) => {
                                    tp.replace(ip + 1);
                                    let blkiknk = blocktokens.pop().unwrap();
                                    match blkiknk {
                                        BlockToken::İkiNoktaNokta(iknkip) => {
                                            let iknk = parsed.get_mut(iknkip).unwrap();
                                            match iknk.typ {
                                                TokenType::İkiNoktaNokta => iknkip,
                                                _ => {
                                                    let o = iknk.clone();
                                                    return Err(match get_lang() {
                                                    SupportedLanguage::Turkish => ErrorGenerator::error(
                                                        "SözdizimHatası",
                                                            &format!(
                                                                "kapatılmamış blok {:?}",
                                                                o.repr()
                                                            ),
                                                            o.line, o.col, o.file,
                                                            None,
                                                        ),
                                                        SupportedLanguage::English => ErrorGenerator::error(
                                                            "SyntaxError",
                                                            &format!(
                                                                "unclosed block {:?}",
                                                                o.repr()
                                                            ),
                                                            o.line, o.col, o.file,
                                                            None,
                                                        ),
                                                    });
                                                },
                                            }
                                        }
                                        a => {
                                            let o = parsed.get(a.unwrap_inner()).unwrap().clone();
                                            return Err(match get_lang() {
                                                SupportedLanguage::Turkish => ErrorGenerator::error(
                                                    "SözdizimHatası",
                                                    &format!(
                                                        "kapatılmamış blok {:?}",
                                                        a
                                                    ),
                                                    o.line, o.col, o.file,
                                                    None,
                                                ),
                                                SupportedLanguage::English => ErrorGenerator::error(
                                                    "SyntaxError",
                                                    &format!(
                                                        "unclosed block {:?}",
                                                        a
                                                    ),
                                                    o.line, o.col, o.file,
                                                    None,
                                                ),
                                            });
                                        },
                                    }
                                }
                                _ => unreachable!(),
                            };
                            parsed.push(Token::new(
                                TokenType::Son { tp },
                                ptoken.line,
                                ptoken.col,
                                ptoken.file.clone(),
                            ));
                        }
                        BlockToken::İşlev(bip) => {
                            let işlev = parsed.get_mut(bip).unwrap();
                            match işlev.typ {
                                TokenType::İşlev { ref mut sonloc } => {
                                    sonloc.replace(ip);
                                }
                                _ => unreachable!(),
                            }
                            parsed.push(Token::new(
                                TokenType::İşlevSonlandır { tp: vec![] },
                                ptoken.line,
                                ptoken.col,
                                ptoken.file.clone(),
                            ));
                            let srets: Vec<usize> = rets.pop().unwrap();
                            for u in srets.iter() {
                                let sr = parsed.get_mut(*u).unwrap();
                                match sr.typ {
                                    TokenType::Ver { ref mut tp } => {
                                        *tp = Some(ip);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        BlockToken::Blok(_) => {
                            parsed.push(Token::new(
                                TokenType::BlokSonlandır,
                                ptoken.line,
                                ptoken.col,
                                ptoken.file.clone(),
                            ));
                        }
                        _ => unimplemented!(),
                    };
                }
                LexTokenType::Doğru => parsed.push(Token::new(
                    TokenType::Bool { val: true },
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Yanlış => parsed.push(Token::new(
                    TokenType::Bool { val: false },
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Artı => parsed.push(Token::new(
                    TokenType::Artı,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::ArtıArtı => parsed.push(Token::new(
                    TokenType::ArtıArtı,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Eksi => parsed.push(Token::new(
                    TokenType::Eksi,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::EksiEksi => parsed.push(Token::new(
                    TokenType::EksiEksi,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Çarpı => parsed.push(Token::new(
                    TokenType::Çarpı,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Bölü => parsed.push(Token::new(
                    TokenType::Bölü,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Modulo => parsed.push(Token::new(
                    TokenType::Modulo,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::De => parsed.push(Token::new(
                    TokenType::De,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Girdi => parsed.push(Token::new(
                    TokenType::Girdi,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Kopya => parsed.push(Token::new(
                    TokenType::Kopya,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Koy => parsed.push(Token::new(
                    TokenType::Koy,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Büyüktür => parsed.push(Token::new(
                    TokenType::Büyüktür,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::BüyükEşittir => parsed.push(Token::new(
                    TokenType::BüyükEşittir,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Küçüktür => parsed.push(Token::new(
                    TokenType::Küçüktür,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::KüçükEşittir => parsed.push(Token::new(
                    TokenType::KüçükEşittir,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Eşittir => parsed.push(Token::new(
                    TokenType::Eşittir,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::EşitDeğildir => parsed.push(Token::new(
                    TokenType::EşitDeğildir,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Değildir => parsed.push(Token::new(
                    TokenType::Değildir,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Takas => parsed.push(Token::new(
                    TokenType::Takas,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Döndür => parsed.push(Token::new(
                    TokenType::Döndür,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Üst => parsed.push(Token::new(
                    TokenType::Üst,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Ve => parsed.push(Token::new(
                    TokenType::Ve,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Veya => parsed.push(Token::new(
                    TokenType::Veya,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::Tipinde => parsed.push(Token::new(
                    TokenType::Tipinde,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
                LexTokenType::EOF => parsed.push(Token::new(
                    TokenType::EOF,
                    ptoken.line,
                    ptoken.col,
                    ptoken.file.clone(),
                )),
            }
        }
        Ok(parsed)
    }
}
