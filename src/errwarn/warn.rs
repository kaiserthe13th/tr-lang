pub trait TrLangWarning {
    pub fn header(&self)  ->  String;
    pub fn content(&self) ->  String;
    pub fn extras(&self)  ->  String;
}

pub struct TrLangWarn {
    typ : String,
    exp : String,
    file: String,
    line: usize,
    col : usize,
}

impl TrLangWarn {
    pub fn new(typ: String, exp: String, file: String, line: usize, col: usize) -> Self {
        Self {
            typ, exp, file, line, col
        }
    }
}

impl TrLangWarning for TrLangWarn {
    pub fn header(&self) -> String {
        format!("[ERROR] on Line {:?}, Column {:?} in {}", self.line, self.col, self.file)
    }
    pub fn content(&self) -> String {
        format!("    {}: {}", self.typ, self.exp)
    }
    pub fn extras(&self) -> String { "".to_string() }
}