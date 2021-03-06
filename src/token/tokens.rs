use serde::{Deserialize, Serialize};

pub mod tokentypes {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        Sına,
        Kopya,
        Takas,
        Döndür,
        Üst,
        Ve,
        Veya,
        Girdi,
        İkiNoktaNokta,
        İkiNokta,
        İken(Option<usize>),
        Sayı { val: f64 },
        Bool { val: bool },
        İse(Option<usize>),
        Yoksa(Option<usize>),
        Son { tp: usize },
        Identifier { id: String },
        İşlev { sonloc: Option<usize> },
        İşlevSonlandır { tp: Vec<usize> },
        Koy,
        Ver { tp: Option<usize> },
        At,
        Tipinde,
        ParenL,
        Hiç,
        Blok,
        BlokSonlandır,
        InScopeParentL,
        InScopeParentR,
        LibSymbol(String),
        EOF,
    }

    #[derive(Debug, Clone)]
    pub enum LexerTokenType {
        ParenL,
        ParenR,
        Comma,
        İşlev,
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
        İkiNokta,
        BüyükEşittir,
        KüçükEşittir,
        Eşittir,
        EşitDeğildir,
        Değildir,
        Sına,
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
        At,
        Ver,
        Yükle,
        Tipinde,
        Hiç,
        Blok,
        InScopeParentL,
        InScopeParentR,
        LibSymbol(String),
        EOF,
    }
}

#[derive(Debug, Clone)]
pub enum Precedence {
    Precedence(usize),
    Reserved,
    ParenL,
    ParenR,
    Comma,
    None,
}

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub typ: tokentypes::LexerTokenType,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
    pub file: String,
    pub precedence: Precedence,
}

impl LexerToken {
    pub fn new(
        typ: tokentypes::LexerTokenType,
        lexeme: String,
        line: usize,
        col: usize,
        file: String,
        precedence: Precedence,
    ) -> Self {
        Self {
            typ,
            lexeme,
            line,
            col,
            file,
            precedence,
        }
    }
    pub fn repr(&self) -> String {
        use tokentypes::LexerTokenType as TokTyp;
        match self.typ {
            TokTyp::Artı => "+".to_string(),
            TokTyp::ArtıArtı => "++".to_string(),
            TokTyp::At => "at".to_string(),
            TokTyp::Doğru => "doğru".to_string(),
            TokTyp::Yanlış => "yanlış".to_string(),
            TokTyp::Bölü => "/".to_string(),
            TokTyp::BüyükEşittir => ">=".to_string(),
            TokTyp::Büyüktür => ">".to_string(),
            TokTyp::De => "de".to_string(),
            TokTyp::Değildir => "!".to_string(),
            TokTyp::Döndür => "dön".to_string(),
            TokTyp::Eksi => "-".to_string(),
            TokTyp::EksiEksi => "--".to_string(),
            TokTyp::EOF => "EOF".to_string(),
            TokTyp::EşitDeğildir => "!=".to_string(),
            TokTyp::Eşittir => "=".to_string(),
            TokTyp::Girdi => "girdi".to_string(),
            TokTyp::Sına => "sına".to_string(),
            TokTyp::Identifier => self.lexeme.clone(),
            TokTyp::İken => "iken".to_string(),
            TokTyp::İkiNoktaNokta => ":.".to_string(),
            TokTyp::İse => "ise".to_string(),
            TokTyp::İşlev => "işlev".to_string(),
            TokTyp::Kopya => "kpy".to_string(),
            TokTyp::Koy => "->".to_string(),
            TokTyp::KüçükEşittir => "<=".to_string(),
            TokTyp::Küçüktür => "<".to_string(),
            TokTyp::Modulo => "%".to_string(),
            TokTyp::Sayı => {
                let val: f64 = self.lexeme.parse().unwrap();
                match val.fract() == 0.0 {
                    true => format!("{:.0?}", val),
                    false => format!("{:?}", val),
                }
            }
            TokTyp::Son => "son".to_string(),
            TokTyp::Takas => "tks".to_string(),
            TokTyp::Tipinde => "@".to_string(),
            TokTyp::Ve => "ve".to_string(),
            TokTyp::Veya => "veya".to_string(),
            TokTyp::Yazı => format!("{:?}", self.lexeme),
            TokTyp::Yoksa => "yoksa".to_string(),
            TokTyp::Çarpı => "*".to_string(),
            TokTyp::Üst => "üst".to_string(),
            TokTyp::Ver => "ver".to_string(),
            TokTyp::ParenL => "(".to_string(),
            TokTyp::ParenR => ")".to_string(),
            TokTyp::Comma => ",".to_string(),
            TokTyp::Yükle => "yükle".to_string(),
            TokTyp::LibSymbol(_) => "yükle".to_string(),
            TokTyp::Hiç => "hiç".to_string(),
            TokTyp::Blok => "blok".to_string(),
            TokTyp::İkiNokta => ":".to_string(),
            TokTyp::InScopeParentL => "(".to_string(),
            TokTyp::InScopeParentR => ")".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserToken {
    pub typ: tokentypes::ParserTokenType,
    pub line: usize,
    pub col: usize,
    pub file: String,
}

impl ParserToken {
    pub fn new(typ: tokentypes::ParserTokenType, line: usize, col: usize, file: String) -> Self {
        Self {
            typ,
            line,
            col,
            file,
        }
    }

    pub fn repr(&self) -> String {
        use tokentypes::ParserTokenType as TokTyp;
        match self.typ {
            TokTyp::Artı => "+".to_string(),
            TokTyp::ArtıArtı => "++".to_string(),
            TokTyp::At => "at".to_string(),
            TokTyp::Bool { val } => match val {
                true => "doğru".to_string(),
                false => "yanlış".to_string(),
            },
            TokTyp::Bölü => "/".to_string(),
            TokTyp::BüyükEşittir => ">=".to_string(),
            TokTyp::Büyüktür => ">".to_string(),
            TokTyp::De => "de".to_string(),
            TokTyp::Değildir => "!".to_string(),
            TokTyp::Döndür => "dön".to_string(),
            TokTyp::Sına => "sına".to_string(),
            TokTyp::Eksi => "-".to_string(),
            TokTyp::EksiEksi => "--".to_string(),
            TokTyp::EOF => "EOF".to_string(),
            TokTyp::EşitDeğildir => "!=".to_string(),
            TokTyp::Eşittir => "=".to_string(),
            TokTyp::Girdi => "girdi".to_string(),
            TokTyp::Identifier { ref id } => id.clone(),
            TokTyp::İken(_) => "iken".to_string(),
            TokTyp::İkiNoktaNokta => ":.".to_string(),
            TokTyp::İse(_) => "ise".to_string(),
            TokTyp::İşlev { .. } => "işlev".to_string(),
            TokTyp::İşlevSonlandır { .. } => "son".to_string(),
            TokTyp::Kopya => "kpy".to_string(),
            TokTyp::Koy => "->".to_string(),
            TokTyp::KüçükEşittir => "<=".to_string(),
            TokTyp::Küçüktür => "<".to_string(),
            TokTyp::Modulo => "%".to_string(),
            TokTyp::Sayı { val } => match val.fract() == 0.0 {
                true => format!("{:.0?}", val),
                false => format!("{:?}", val),
            },
            TokTyp::Son { .. } => "son".to_string(),
            TokTyp::Takas => "tks".to_string(),
            TokTyp::Tipinde => "@".to_string(),
            TokTyp::Ve => "ve".to_string(),
            TokTyp::Veya => "veya".to_string(),
            TokTyp::Yazı { ref val } => format!("{:?}", val),
            TokTyp::Yoksa(_) => "yoksa".to_string(),
            TokTyp::Çarpı => "*".to_string(),
            TokTyp::Üst => "üst".to_string(),
            TokTyp::Ver { .. } => "ver".to_string(),
            TokTyp::ParenL => "(".to_string(),
            TokTyp::Hiç => "hiç".to_string(),
            TokTyp::Blok => "blok".to_string(),
            TokTyp::BlokSonlandır => "son".to_string(),
            TokTyp::İkiNokta => ":".to_string(),
            TokTyp::InScopeParentL => "(".to_string(),
            TokTyp::InScopeParentR => ")".to_string(),
            TokTyp::LibSymbol(_) => "yükle".to_string(),
        }
    }
}
