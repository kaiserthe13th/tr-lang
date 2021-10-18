use crate::token::LexerToken as LexToken;
use crate::token::tokentypes::LexerTokenType as LexTokenType;

use crate::token::ParserToken as Token;
use crate::token::tokentypes::ParserTokenType as TokenType;

enum BlockToken {
    İse(usize),
    İken(usize),
    İkiNoktaNokta(usize),
}

pub struct Parser {
    tokens : Vec<LexToken>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
    pub fn parse(&mut self) -> Vec<Token> {
        let mut parsed: Vec<Token> = vec![];
        let mut blocktokens: Vec<BlockToken> = vec![];

        for ptoken in self.tokens.iter().enumerate() {
            match &ptoken.typ {
                LexTokenType::Sayı => {
                    parsed.push(Token::new(
                        TokenType::Sayı { val: ptoken.lexeme.as_str().parse() },
                        ptoken.line,
                        ptoken.column,
                    ))
                },
                LexTokenType::Yazı => {
                    parsed.push(Token::new(
                        TokenType::Yazı { val: ptoken.lexeme },
                        ptoken.line,
                        ptoken.column,
                    ))
                },
                LexTokenType::İken => {
                    blocktokens.push(BlockToken::İken(ptoken))
                    parsed.push(Token::new(
                        TokenType::İken { yoksa: None },
                        ptoken.line,
                        ptoken.column,
                    ))
                },
                LexTokenType::İse => {
                    parsed.push(Token::new(
                        TokenType::İse { yoksa: None },
                        ptoken.line,
                        ptoken.column,
                    ))
                },
                LexTokenType::Yoksa => {
                    parsed.push(Token::new(
                        TokenType::Yoksa { tp: None },
                        ptoken.line,
                        ptoken.column,
                    ))
                },
            }
        }
    }
}