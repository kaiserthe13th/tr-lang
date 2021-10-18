pub trait TrLangError {
    pub fn header(&self)  ->  String;
    pub fn content(&self) ->  String;
    pub fn extras(&self)  ->  String;
}

pub struct TrLangErr {
    typ : String,
    exp : String,
    file: String,
    line: usize,
    col : usize,
}

impl TrLangErr {
    pub fn new(typ: String, exp: String, file: String, line: usize, col: usize) -> Self {
        Self {
            typ, exp, file, line, col
        }
    }
}

impl TrLangError for TrLangErr {
    pub fn header(&self) -> String {
        format!("[ERROR] on Line {:?}, Column {:?} in {}", self.line, self.col, self.file)
    }
    pub fn content(&self) -> String {
        format!("    {}: {}", self.typ, self.exp)
    }
    pub fn extras(&self) -> String { "".to_string() }
}