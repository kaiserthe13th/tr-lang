use crate::errwarn::ErrorGenerator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Run;
use crate::store::VERSION;
use crate::util::{get_lang, SupportedLanguage};
use lazy_static::lazy_static;
use regex::Regex;
use rustyline::completion::Completer;
use rustyline::completion::Pair;
use rustyline::error::ReadlineError;
use rustyline::Editor;

struct InteractiveCompleter;
impl Completer for InteractiveCompleter {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"[^()\d,'"+\-*/><!=%?.@\s][^\s"':?=<>!/%*@,()]*"#).unwrap();
            static ref KNOWN_KEYWORDS: Vec<&'static str> = vec![
                "at", "ver", "de", "ise", "son", "iken", "yoksa", "doğru", "yanlış", "kpy", "tks",
                "üst", "veya", "ve", "dön", "girdi", "işlev", "yükle",
            ];
        }
        let matches = RE.find_iter(line);
        for m in matches.into_iter() {
            if m.end() == pos {
                return Ok((
                    m.start(),
                    KNOWN_KEYWORDS
                        .iter()
                        .filter(|a| a.starts_with(m.as_str()))
                        .map(|a| Pair {
                            display: a.to_string(),
                            replacement: a.to_string(),
                        })
                        .collect(),
                ));
            }
        }
        Ok((0, Vec::with_capacity(0)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum QuietLevel {
    None,
    Quiet,
    Quieter,
    Quietest,
}
impl QuietLevel {
    pub fn inc(&mut self) {
        match self {
            Self::None => *self = Self::Quiet,
            Self::Quiet => *self = Self::Quieter,
            Self::Quieter => *self = Self::Quietest,
            Self::Quietest => (),
        }
    }
    pub fn inc_by(&mut self, i: usize) {
        for _ in 0..i {
            self.inc()
        }
    }
}

pub struct Interactive {
    line: usize,
    quiet: QuietLevel,
}
impl Default for Interactive {
    fn default() -> Self {
        Self {
            line: 1,
            quiet: QuietLevel::None,
        }
    }
}
impl Interactive {
    pub fn new(quiet: QuietLevel) -> Self {
        Self {
            quiet,
            ..Default::default()
        }
    }
    pub fn start(&mut self) {
        if self.quiet == QuietLevel::None {
            match get_lang() {
                SupportedLanguage::Turkish => {
                    println!("tr-lang ({VERSION}) interaktif konsol");
                    println!("çıkmak için `#çık` yazın");
                    println!("yürütmek için `#yürüt` yazın");
                }
                SupportedLanguage::English => {
                    println!("tr-lang ({VERSION}) interactive console");
                    println!("type `#çık` to exit");
                    println!("type `#yürüt` to run");
                }
            }
        }
        let mut fbuf = String::new();
        let mut editor = Editor::<()>::new();
        if editor.load_history(".trlhistory").is_err() {
            match get_lang() {
                SupportedLanguage::Turkish => println!("Tarih bulunamadı."),
                SupportedLanguage::English => println!("No previous history."),
            }
        }
        loop {
            let pr = match self.quiet {
                QuietLevel::None => format!("trli:{:03}#> ", self.line),
                QuietLevel::Quiet => format!("{:03}#> ", self.line),
                QuietLevel::Quieter => "#> ".to_string(),
                QuietLevel::Quietest => "".to_string(),
            };
            let rl = editor.readline(&pr);
            match rl {
                Ok(buf) => match buf.as_str() {
                    "#çık" => break,
                    "#yürüt" => {
                        let (mut memcs, _) = Run::new(
                            Parser::from_lexer(&mut Lexer::new(fbuf), ".".to_string()).parse(),
                        )
                        .run("<trli>".to_string(), None, true)
                        .unwrap_or_else(|(s, h, e)| {
                            e.eprint();
                            (s, h)
                        });
                        println!();
                        if memcs.len() > 0 {
                            println!("=> {:?}", memcs.iter_vec());
                        }
                        fbuf = String::new();
                        self.line = 1;
                    }
                    _ => {
                        editor.add_history_entry(&buf);
                        fbuf.push_str(&buf);
                        fbuf.push('\n');
                        self.line += 1;
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    eprintln!("Ctrl+C");
                }
                Err(ReadlineError::Eof) => break,
                Err(e) => match get_lang() {
                    SupportedLanguage::Turkish => ErrorGenerator::error(
                        "EditörHatası",
                        &format!("{}", e),
                        self.line,
                        0,
                        "<trli>".to_string(),
                        None,
                    ),
                    SupportedLanguage::English => ErrorGenerator::error(
                        "EditorError",
                        &format!("{}", e),
                        self.line,
                        0,
                        "<trli>".to_string(),
                        None,
                    ),
                }
                .eprint(),
            }
        }
        editor.save_history(".trlhistory").unwrap();
    }
}
