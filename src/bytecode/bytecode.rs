use crate::token::ParserToken;
use bincode;

pub fn to_bytecode(parsed: Vec<ParserToken>) -> Vec<u8> {
    bincode::serialize(&parsed).unwrap()
}

pub fn from_bytecode(bytecode: &[u8]) -> Vec<ParserToken> {
    bincode::deserialize(bytecode).unwrap()
}
