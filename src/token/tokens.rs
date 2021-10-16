pub mod tokentypes {
    #[derive(Debug)]
    pub enum TokenType {
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
        Sor,
        SorYoksa,
        SorSon,
    }
}

#[derive(Debug)]
pub struct Token {
    typ: tokentypes::TokenType,
    lexeme: String,
    line:    usize,
    col:     usize,
}

impl Token {
    pub fn new(typ: tokentypes::TokenType, lexeme: String, line: usize, col: usize) -> Self {
        Self { typ, lexeme, line, col }
    }
}