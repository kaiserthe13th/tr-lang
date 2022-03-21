use std::collections::HashMap;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::{Run, RunConfig};
use crate::store::VERSION;
use crate::util::{get_lang, SupportedLanguage};
use lazy_static::lazy_static;
use regex::Regex;
use rustyline::completion::Completer;
use rustyline::completion::Pair;
use rustyline::error::ReadlineError;
use rustyline::Editor;

// TODO: Finish completer
/// Completer in progress
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

#[derive(Debug)]
pub struct InteractiveOptions {
    reset_buf_on_run: bool,
    aliases: HashMap<String, String>,
}
impl Default for InteractiveOptions {
    fn default() -> Self {
        Self {
            reset_buf_on_run: true,
            aliases: HashMap::new(),
        }
    }
}

pub struct Interactive {
    line: usize,
    quiet: QuietLevel,
    opt: InteractiveOptions,
}
impl Default for Interactive {
    fn default() -> Self {
        Self {
            line: 1,
            quiet: QuietLevel::None,
            opt: InteractiveOptions::default(),
        }
    }
}
impl Interactive {
    pub fn new(quiet: QuietLevel, opt: InteractiveOptions) -> Self {
        Self {
            quiet,
            opt,
            ..Default::default()
        }
    }
    pub fn resolve_alias(&self, alias: &String) -> Option<String> {
        match self.opt.aliases.get(alias) {
            Some(al) => {
                if matches!(al.as_str(), "yürüt" | "çık" | "ayar" | "takma-ad" | "reset-buf") {
                    Some(al.clone())
                } else {
                    self.resolve_alias(al)
                }
            },
            None => if matches!(alias.as_str(), "yürüt" | "çık" | "ayar" | "takma-ad" | "reset-buf") {
                Some(alias.clone())
            } else {
                None
            },
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
                Ok(buf) => {
                    if buf.starts_with('#') {
                        let (cmd, args): (String, Vec<String>) = {
                            let mut t = buf[1..].split_whitespace().map(String::from);
                            let cmd = t.next().unwrap();
                            (cmd, t.collect())
                        };
                        match self.resolve_alias(&cmd) {
                            Some(c) => match c.as_str() {
                                "çık" => break,
                                "yürüt" => {
                                    let (mut memcs, _) = Run::new(
                                        match match Parser::from_lexer(&mut Lexer::new(fbuf.clone()), ".".to_string()) {
                                            Ok(parser) => parser,
                                            Err(e) => {
                                                e.error_print();
                                                continue;
                                            }
                                        }.parse() {
                                            Ok(ptk) => ptk,
                                            Err(e) => { e.error_print(); continue; }
                                        },
                                    )
                                    .run(RunConfig {
                                        file: "<trli>".to_string(),
                                        repl: true,
                                        ..Default::default()
                                    })
                                    .unwrap_or_else(|(s, h, e)| {
                                        e.error_print();
                                        (s, h)
                                    });
                                    println!();
                                    if memcs.len() > 0 {
                                        println!("=> {:?}", memcs.iter_vec());
                                    }
                                    if self.opt.reset_buf_on_run {
                                        fbuf = String::new();
                                        self.line = 0;
                                    }
                                },
                                "takma-ad" => {
                                    if args.len() == 1 {
                                        if let Some(al) = self.resolve_alias(&args[0]) {
                                            println!("{} -> {al}", args[0]);
                                        } else {
                                            match get_lang() {
                                                SupportedLanguage::Turkish => eprintln!("takma-ad: {} bulunamadı.", args[0]),
                                                SupportedLanguage::English => eprintln!("takma-ad: {} not found.", args[0]),
                                            }
                                        }
                                    } else if args.len() >= 2 {
                                        self.opt.aliases.insert(args[0].clone(), args[1].clone());
                                    } else {
                                        match get_lang() {
                                            SupportedLanguage::Turkish => eprintln!("takma-ad: gereğinden daha az argüman"),
                                            SupportedLanguage::English => eprintln!("takma-ad: less arguments than required"),
                                        }
                                    }
                                },
                                "reset-buf" => {
                                    fbuf = String::new();
                                    self.line = 0;
                                },
                                "ayar" => todo!(), // TODO
                                _ => continue,
                            },
                            None => continue,
                        }
                    }
                    editor.add_history_entry(&buf);
                    fbuf.push_str(&buf);
                    fbuf.push('\n');
                    self.line += 1;
                }
                Err(ReadlineError::Interrupted) => {
                    eprintln!("Ctrl+C");
                }
                Err(ReadlineError::Eof) => break,
                Err(e) => match get_lang() {
                    SupportedLanguage::Turkish => Error::new(
                        "EditörHatası",
                        &format!("{}", e),
                        vec![(self.line, 0, "<trli>".to_string(), None)],
                        None,
                    ),
                    SupportedLanguage::English => Error::new(
                        "EditorError",
                        &format!("{}", e),
                        vec![(self.line, 0, "<trli>".to_string(), None)],
                        None,
                    ),
                }
                .error_print(),
            }
        }
        editor.save_history(".trlhistory").unwrap();
    }
}
