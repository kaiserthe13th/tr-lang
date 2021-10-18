use crate::lexer;

use crate::token::LexerToken as LexToken;
use crate::token::tokentypes::LexerTokenType as LexTokenType;

use crate::token::ParserToken as Token;
use crate::token::tokentypes::ParserTokenType as TokenType;

#[derive(Debug)]
enum BlockToken {
    İse(usize),
    İken(usize),
    İkiNoktaNokta(usize),
}

#[derive(Clone)]
pub struct Parser {
    tokens : Vec<LexToken>,
}

impl Parser {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self {
            tokens,
        }
    }

    pub fn from_lexer(lexer: &mut lexer::Lexer) -> Self {
        Self {
            tokens: lexer.lex(),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut parsed: Vec<Token> = vec![];
        let mut blocktokens: Vec<BlockToken> = vec![];

        for (ip, ptoken) in self.tokens.iter().enumerate() {
            // #[allow(unused)]
            match ptoken.typ {
                LexTokenType::Sayı => {
                    parsed.push(Token::new(
                        TokenType::Sayı { val: ptoken.lexeme.as_str().parse().unwrap() },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Yazı => {
                    parsed.push(Token::new(
                        TokenType::Yazı { val: ptoken.lexeme.clone() },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Identifier => {
                    parsed.push(Token::new(
                        TokenType::Identifier { id: ptoken.lexeme.clone() },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::İkiNoktaNokta => {
                    blocktokens.push(BlockToken::İkiNoktaNokta(ip));
                    parsed.push(Token::new(
                        TokenType::İkiNoktaNokta,
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::İken => {
                    let last_blocktoken = blocktokens.pop().unwrap();
                    let tp = match last_blocktoken {
                        BlockToken::İkiNoktaNokta(bip) => {
                            bip
                        },
                        _ => unimplemented!(),
                    };
                    blocktokens.push(BlockToken::İken(tp));
                    parsed.push(Token::new(
                        TokenType::İken(None),
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::İse => {
                    blocktokens.push(BlockToken::İse(ip));
                    parsed.push(Token::new(
                        TokenType::İse( None ),
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Yoksa => {
                    let last_blocktoken = blocktokens.pop().unwrap();
                    let tp = match last_blocktoken {
                        BlockToken::İse(bip) => {
                            let ise = &mut parsed[bip];
                            match ise.typ {
                                TokenType::İse ( mut yoksa ) => {
                                    yoksa = Some(ip + 1);
                                },
                                _ => unreachable!(),
                            }
                            ip + 1
                        },
                        _ => unimplemented!(),
                    };
                    blocktokens.push(BlockToken::İse(ip));
                    parsed.push(Token::new(
                        TokenType::Yoksa(None),
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Son => {
                    let last_blocktoken = blocktokens.pop().unwrap();
                    let tp = match last_blocktoken {
                        BlockToken::İse(bip) => {
                            let ise = &mut parsed[bip];
                            match ise.typ {
                                TokenType::İse ( mut yoksa ) => {
                                    yoksa = Some(ip);
                                },
                                TokenType::Yoksa ( mut tp ) => {
                                    tp = Some(ip);
                                },
                                _ => unreachable!(),
                            }
                            ip + 1
                        },
                        BlockToken::İken(bip) => {
                            let iken = parsed.get_mut(bip).unwrap();
                            match iken.typ {
                                TokenType::İken ( mut yoksa ) => {
                                    yoksa = Some(ip + 1);
                                },
                                _ => unreachable!(),
                            }
                            bip
                        },
                        _ => unimplemented!(),
                    };
                    parsed.push(Token::new(
                        TokenType::Son { tp },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Doğru => {
                    parsed.push(Token::new(
                        TokenType::Bool { val: true },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Yanlış => {
                    parsed.push(Token::new(
                        TokenType::Bool { val: false },
                        ptoken.line,
                        ptoken.col,
                    ))
                },
                LexTokenType::Artı => {
                    parsed.push(Token::new(TokenType::Artı, ptoken.line, ptoken.col))
                },
                LexTokenType::ArtıArtı => {
                    parsed.push(Token::new(TokenType::ArtıArtı, ptoken.line, ptoken.col))
                },
                LexTokenType::Eksi => {
                    parsed.push(Token::new(TokenType::Eksi, ptoken.line, ptoken.col))
                },
                LexTokenType::EksiEksi => {
                    parsed.push(Token::new(TokenType::EksiEksi, ptoken.line, ptoken.col))
                },
                LexTokenType::Çarpı => {
                    parsed.push(Token::new(TokenType::Çarpı, ptoken.line, ptoken.col))
                },
                LexTokenType::Bölü => {
                    parsed.push(Token::new(TokenType::Bölü, ptoken.line, ptoken.col))
                },
                LexTokenType::Modulo => {
                    parsed.push(Token::new(TokenType::Modulo, ptoken.line, ptoken.col))
                }
                LexTokenType::De => {
                    parsed.push(Token::new(TokenType::De, ptoken.line, ptoken.col))
                },
                LexTokenType::Girdi => {
                    parsed.push(Token::new(TokenType::Girdi, ptoken.line, ptoken.col))
                },
                LexTokenType::Kopya => {
                    parsed.push(Token::new(TokenType::Kopya, ptoken.line, ptoken.col))
                },
                LexTokenType::Koy => {
                    parsed.push(Token::new(TokenType::Koy, ptoken.line, ptoken.col))
                },
                LexTokenType::Büyüktür => {
                    parsed.push(Token::new(TokenType::Büyüktür, ptoken.line, ptoken.col))
                },
                LexTokenType::BüyükEşittir => {
                    parsed.push(Token::new(TokenType::BüyükEşittir, ptoken.line, ptoken.col))
                },
                LexTokenType::Küçüktür => {
                    parsed.push(Token::new(TokenType::Küçüktür, ptoken.line, ptoken.col))
                },
                LexTokenType::KüçükEşittir => {
                    parsed.push(Token::new(TokenType::KüçükEşittir, ptoken.line, ptoken.col))
                },
                LexTokenType::Eşittir => {
                    parsed.push(Token::new(TokenType::Eşittir, ptoken.line, ptoken.col))
                },
                LexTokenType::EşitDeğildir => {
                    parsed.push(Token::new(TokenType::EşitDeğildir, ptoken.line, ptoken.col))
                },
                LexTokenType::Değildir => {
                    parsed.push(Token::new(TokenType::Değildir, ptoken.line, ptoken.col))
                },
                LexTokenType::Takas => {
                    parsed.push(Token::new(TokenType::Takas, ptoken.line, ptoken.col))
                },
                LexTokenType::Döndür => {
                    parsed.push(Token::new(TokenType::Döndür, ptoken.line, ptoken.col))
                },
                LexTokenType::Üst => {
                    parsed.push(Token::new(TokenType::Üst, ptoken.line, ptoken.col))
                },
                LexTokenType::Ve => {
                    parsed.push(Token::new(TokenType::Ve, ptoken.line, ptoken.col))
                },
                LexTokenType::Veya => {
                    parsed.push(Token::new(TokenType::Veya, ptoken.line, ptoken.col))
                },
                LexTokenType::EOF => {
                    parsed.push(Token::new(TokenType::EOF, ptoken.line, ptoken.col))
                },
            }
        }
        parsed
    }
}