//! a module that holds tokentypes as its name shows
//! 
//! these tokentypes are used when generating Tokens
pub mod tokentypes {
    #[derive(Debug, Clone)]
    pub enum ParserTokenType {
        Yazı { val: String },
        Artı,
        Eksi,
        ArtıArtı,
        EksiEksi,
        Çarpı,
        Bölü,
        Modulo,
        De,
        Büyüktür,
        Küçüktür,
        BüyükEşittir,
        KüçükEşittir,
        Eşittir,
        EşitDeğildir,
        Değildir,
        Kopya,
        Takas,
        Döndür,
        Üst,
        Ve,
        Veya,
        Girdi,
        İkiNoktaNokta,
        İken ( Option<usize> ),
        Sayı { val: f64 },
        Bool { val: bool },
        İse ( Option<usize> ),
        Yoksa ( Option<usize> ),
        Son { tp: usize },
        Identifier { id: String },
        Koy,
        EOF,
    }
    
    #[derive(Debug, Clone)]
    pub enum LexerTokenType {
        Yazı,
        Sayı,
        De,
        İken,
        İse,
        Yoksa,
        Identifier,
        Son,
        Kopya,
        Doğru,
        Yanlış,
        Artı,
        ArtıArtı,
        Eksi,
        EksiEksi,
        Çarpı,
        Bölü,
        Modulo,
        Büyüktür,
        Küçüktür,
        İkiNoktaNokta,
        BüyükEşittir,
        KüçükEşittir,
        Eşittir,
        EşitDeğildir,
        Değildir,
        // Replaced with `İse` `Yoksa` and `Son`
        // `İse` `Yoksa` ve `Son` ile değiştirildi
        /*
        Sor,
        SorYoksa,
        SorSon,
        */
        Takas,
        Üst,
        Döndür,
        Ve,
        Veya,
        Girdi,
        Koy,
        EOF,
    }
}

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub typ:    tokentypes::LexerTokenType,
    pub lexeme: String,
    pub line:   usize,
    pub col:    usize,
}

impl LexerToken {
    pub fn new(typ: tokentypes::LexerTokenType, lexeme: String, line: usize, col: usize) -> Self {
        Self { typ, lexeme, line, col }
    }
}

#[derive(Debug, Clone)]
pub struct ParserToken {
    pub typ:  tokentypes::ParserTokenType,
    pub line: usize,
    pub col:  usize,
}

impl ParserToken {
    pub fn new(typ: tokentypes::ParserTokenType, line: usize, col: usize) -> Self {
        Self { typ, line, col }
    }
}
