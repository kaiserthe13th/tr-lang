use dlopen::symbor::Library;

use crate::error::Error;
use crate::ffi::{load_library, terminate_library};
use crate::mem::{HashMemory, Map, Object, StackMemory};
use crate::token::{tokentypes::ParserTokenType as TokenType, ParserToken as Token};
use crate::util::{get_lang, SupportedLanguage};
use std::io::{self, prelude::*};

pub struct Run {
    program: Vec<Token>,
    pub(crate) current: usize,
}

pub struct RunConfig {
    pub mem: (StackMemory, HashMemory),
    pub repl: bool,
    pub file: String,
    pub supress_warnings: bool,
}
impl Default for RunConfig {
    fn default() -> Self {
        Self {
            mem: (StackMemory::new(), HashMemory::new()),
            repl: false,
            file: ".".to_string(),
            supress_warnings: true,
        }
    }
}

pub type Trace = (usize, usize, String, Option<String>);

impl Run {
    pub fn new(program: Vec<Token>) -> Self {
        Self {
            program,
            current: 0,
        }
    }

    pub fn run(
        &mut self,
        config: RunConfig,
    ) -> Result<(StackMemory, HashMemory), (StackMemory, HashMemory, Error)> {
        let (mut stack, mut hashs) = config.mem;
        let file = config.file;
        // let mut warnings: Vec<Box<dyn FnOnce()>> = vec![]; // for later use
        let mut current_namespace: Vec<String> = vec![];
        let mut traceback: Vec<Trace> = vec![];
        let mut loaded_libraries: Vec<(Library, String)> = vec![];

        while self.program.len() > self.current {
            let tokenc = self.program.get(self.current).unwrap().clone();
            let token = self.program.get_mut(self.current).unwrap();

            match token.typ.clone() {
                TokenType::LibSymbol(s) => {
                    let lib = load_library(&s, &mut stack, &mut hashs);
                    match lib {
                        Ok(lib) => loaded_libraries.push((lib, s)),
                        Err(e) => return Err((stack, hashs, Error::new(
                            "DinamikYüklemeHatası",
                            &format!("{}", e),
                            traceback,
                            None,
                        ))),
                    }
                    self.current += 1;
                },
                TokenType::InScopeParentL => {
                    stack.new_stack();

                    let tok = self.program.get(self.current + 1).unwrap();
                    match tok.typ {
                        TokenType::Identifier { ref id } => {
                            match hashs.get(id) {
                                Some(o) => stack.push(o.clone()),
                                None => return Err((stack, hashs.clone(), match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BilinmeyenTanımlayıcı",
                                        &format!(
                                            "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                                            tok.repr()
                                        ),
                                        { traceback.push((tok.line, tok.col, tok.file.clone(), None)); traceback },
                                        {
                                            let mut hashk = hashs.clone().into_keys();
                                            hashk.sort();
                                            let n = hashk.binary_search(id).unwrap_err();
                                            if hashk.is_empty() {
                                                None
                                            } else {
                                                Some(format!("`{}` demek mi istediniz?", hashk[n]))
                                            }
                                        }
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BilinmeyenTanımlayıcı",
                                        &format!("unknown identifier: `{}`, this identifier could not be found", tok.repr()),
                                        { traceback.push((tok.line, tok.col, tok.file.clone(), None)); traceback },
                                        {
                                            let mut hashk = hashs.clone().into_keys();
                                            hashk.sort();
                                            let n = hashk.binary_search(id).unwrap_err();
                                            if hashk.is_empty() {
                                                None
                                            } else {
                                                Some(format!("maybe you meant {}?", hashk[n]))
                                            }
                                        },
                                    ),
                                })),
                            }
                        }
                        _ => return Err((stack, hashs, match get_lang() {
                                SupportedLanguage::Turkish => Error::new(
                                    "BeklenmedikSimge",
                                    &format!(
                                        "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                        tok.repr()
                                    ),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                ),
                                SupportedLanguage::English => Error::new(
                                    "BeklenmedikSimge",
                                    &format!(
                                        "expected identifier, but found `{}`",
                                        tok.repr()
                                    ),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                ),
                            },
                        )),
                    }

                    self.current += 2;

                    let mut latest_id = None;
                    while self.program.len() > self.current {
                        let c = self.program.get(self.current).unwrap().clone();

                        match c.typ {
                            TokenType::İkiNokta => {
                                let a = match stack.pop() {
                                    Some(a) => a,
                                    None => return Err((stack, hashs, match get_lang() {
                                        SupportedLanguage::Turkish => {
                                            Error::new(
                                                "KümedeYeterliDeğişkenYok",
                                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            )
                                        }
                                        SupportedLanguage::English => {
                                            Error::new(
                                                "KümedeYeterliDeğişkenYok",
                                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            )
                                        }
                                    })),
                                };
                                match a {
                                    Object::Harita(map) => {
                                        let tok = self.program.get(self.current + 1).unwrap();
                                        match tok.typ {
                                            TokenType::Identifier { ref id } => {
                                                let o = match map.map.get(id) {
                                                    Some(o) => o.clone(),
                                                    None => return Err((stack, hashs.clone(), match get_lang() {
                                                        SupportedLanguage::Turkish => Error::new(
                                                            "BilinmeyenTanımlayıcı",
                                                            &format!(
                                                                "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                                                                tok.repr()
                                                            ),
                                                            { traceback.push((tok.line, tok.col, tok.file.clone(), None)); traceback },
                                                            {
                                                                let mut hashk: Vec<_> = map.map.clone().into_keys().collect();
                                                                hashk.sort();
                                                                let n = hashk.binary_search(id).unwrap_err();
                                                                if hashk.is_empty() {
                                                                    None
                                                                } else {
                                                                    Some(format!("`{}` demek mi istediniz?", hashk[n]))
                                                                }
                                                            }
                                                        ),
                                                        SupportedLanguage::English => Error::new(
                                                            "BilinmeyenTanımlayıcı",
                                                            &format!(
                                                                "unknown identifier: `{}`, this identifier could not be found",
                                                                tok.repr()
                                                            ),
                                                            { traceback.push((tok.line, tok.col, tok.file.clone(), None)); traceback },
                                                            {
                                                                let mut hashk: Vec<_> = map.map.clone().into_keys().collect();
                                                                hashk.sort();
                                                                let n = hashk.binary_search(id).unwrap_err();
                                                                if hashk.is_empty() {
                                                                    None
                                                                } else {
                                                                    Some(format!("maybe you meant {}?", hashk[n]))
                                                                }
                                                            },
                                                        ),
                                                    })),
                                                };
                                                latest_id = Some(id);
                                                stack.push(o);
                                                self.current += 2;
                                            },
                                            TokenType::Çarpı => {
                                                for (k, v) in map.map.iter() {
                                                    hashs.insert(k.clone(), v.clone());
                                                }
                                                let next = match self.program.get(self.current + 2) {
                                                    Some(t) => t,
                                                    None => return Err((
                                                        stack,
                                                        hashs,
                                                        match get_lang() {
                                                            SupportedLanguage::Turkish => Error::new(
                                                                "BeklenmedikSimge",
                                                                &format!(
                                                                    "`)` beklenmişti ancak birşey bulunamadı",
                                                                ),
                                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                                None,
                                                            ),
                                                            SupportedLanguage::English => Error::new(
                                                                "BeklenmedikSimge",
                                                                &format!(
                                                                    "expected `)`, but found nothing",
                                                                ),
                                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                                None,
                                                            ),
                                                        },
                                                    )),
                                                };
                                                match next.typ {
                                                    TokenType::InScopeParentR => {
                                                        self.current += 3;
                                                        break;
                                                    },
                                                    _ => return Err((
                                                        stack,
                                                        hashs,
                                                        match get_lang() {
                                                            SupportedLanguage::Turkish => Error::new(
                                                                "BeklenmedikSimge",
                                                                &format!(
                                                                    "`)` beklenmişti ancak `{}` bulundu",
                                                                    next.repr()
                                                                ),
                                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                                None,
                                                            ),
                                                            SupportedLanguage::English => Error::new(
                                                                "BeklenmedikSimge",
                                                                &format!(
                                                                    "expected `)`, but found `{}`",
                                                                    next.repr()
                                                                ),
                                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                                None,
                                                            ),
                                                        },
                                                    )),
                                                }
                                            },
                                            _ => return Err((
                                                stack,
                                                hashs,
                                                match get_lang() {
                                                    SupportedLanguage::Turkish => Error::new(
                                                        "BeklenmedikSimge",
                                                        &format!(
                                                            "`)` beklenmişti ancak `{}` bulundu",
                                                            tok.repr()
                                                        ),
                                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                        None,
                                                    ),
                                                    SupportedLanguage::English => Error::new(
                                                        "BeklenmedikSimge",
                                                        &format!(
                                                            "expected `)`, but found `{}`",
                                                            tok.repr()
                                                        ),
                                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                        None,
                                                    ),
                                                },
                                            )),
                                        }
                                    }
                                    b => return Err((
                                        stack,
                                        hashs,
                                        match get_lang() {
                                            SupportedLanguage::Turkish => Error::new(
                                                "BeklenmedikTip",
                                                &format!("harita beklenmişti ancak `{:?}` bulundu", b),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            ),
                                            SupportedLanguage::English => Error::new(
                                                "BeklenmedikTip",
                                                &format!("expected map but found `{:?}`", b),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            ),
                                        },
                                    )),
                                }
                            }
                            TokenType::InScopeParentR => {
                                self.current += 1;
                                if let Some(id) = latest_id {
                                    hashs.insert(id.clone(), stack.pop().unwrap());
                                } else { unreachable!() }
                                break;
                            }
                            _ => todo!(),
                        }
                    }
                    stack.del_stack();
                }
                TokenType::InScopeParentR => {},
                TokenType::İkiNokta => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                            "KümedeYeterliDeğişkenYok",
                            &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                            None,
                        )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                            "KümedeYeterliDeğişkenYok",
                            &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                            None,
                        )
                            }
                        })),
                    };
                    match a {
                        Object::Harita(map) => {
                            let id = self.program.get(self.current + 1).unwrap();
                            match id.typ.clone() {
                                TokenType::Identifier { id: ident } => {
                                    let o = match map.map.get(&ident) {
                                        Some(o) => o,
                                        None => return Err((stack, hashs.clone(), match get_lang() {
                                            SupportedLanguage::Turkish => Error::new(
                                                "BilinmeyenTanımlayıcı",
                                                &format!(
                                                    "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                                                    tokenc.repr()
                                                ),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                {
                                                    let mut hashk = hashs.clone().into_keys();
                                                    hashk.sort();
                                                    let n = hashk.binary_search(&ident).unwrap_err();
                                                    if hashk.is_empty() {
                                                        None
                                                    } else {
                                                        Some(format!("`{}` demek mi istediniz?", hashk[n]))
                                                    }
                                                }
                                            ),
                                            SupportedLanguage::English => Error::new(
                                                "BilinmeyenTanımlayıcı",
                                                &format!("unknown identifier: `{}`, this identifier could not be found", tokenc.repr()),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                {
                                                    let mut hashk = hashs.clone().into_keys();
                                                    hashk.sort();
                                                    let n = hashk.binary_search(&ident).unwrap_err();
                                                    if hashk.is_empty() {
                                                        None
                                                    } else {
                                                        Some(format!("maybe you meant {}?", hashk[n]))
                                                    }
                                                },
                                            ),
                                        })),
                                    };
                                    match o {
                                        Object::Hiç
                                        | Object::Bool(_)
                                        | Object::Sayı(_)
                                        | Object::Yazı(_)
                                        | Object::Liste(_)
                                        | Object::FfiObject(_)
                                        | Object::Harita(_) => {
                                            stack.push(o.clone());
                                            self.current += 2;
                                        }
                                        Object::FfiFunction(f) => {
                                            let res = f.call(&mut stack, &mut hashs);
                                            match res {
                                                Ok(Some(o)) => stack.push(o),
                                                Ok(_) => (),
                                                Err(e) => return Err((stack, hashs, e)),
                                            }
                                            self.current += 2;
                                        }
                                        Object::İşlev(tp) => {
                                            let işlev = self.program.get(*tp).unwrap();
                                            match işlev.typ {
                                                TokenType::İşlev { sonloc: tpi } => {
                                                    let loc = match tpi {
                                                        Some(i) => i,
                                                        None => unreachable!(),
                                                    };
                                                    let işlevson =
                                                        self.program.get_mut(loc).unwrap();
                                                    match &mut işlevson.typ {
                                                        TokenType::İşlevSonlandır {
                                                            tp: ref mut tps,
                                                        } => {
                                                            tps.push(self.current + 1);
                                                        }
                                                        _ => unreachable!(),
                                                    }
                                                    self.current = *tp + 2;
                                                }
                                                _ => unreachable!(),
                                            }
                                            if let TokenType::Identifier { id : fname } =
                                                &self.program.get(*tp + 1).unwrap().typ {
                                                traceback.push((tokenc.line, tokenc.col, tokenc.file, Some(fname.clone())));
                                            }
                                            stack.new_stack();
                                            hashs.new_hash();
                                        }
                                    }
                                }
                                _ => {
                                    return Err((
                                        stack,
                                        hashs,
                                        match get_lang() {
                                            SupportedLanguage::Turkish => Error::new(
                                                "BeklenmedikSimge",
                                                &format!(
                                                    "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                                    id.repr()
                                                ),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            ),
                                            SupportedLanguage::English => Error::new(
                                                "BeklenmedikSimge",
                                                &format!(
                                                    "expected identifier, but found `{}`",
                                                    id.repr()
                                                ),
                                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                None,
                                            ),
                                        },
                                    ))
                                }
                            }
                        }
                        b => {
                            return Err((
                                stack,
                                hashs,
                                match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BeklenmedikTip",
                                        &format!("harita beklenmişti ancak `{:?}` bulundu", b),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BeklenmedikTip",
                                        &format!("expected map but found `{:?}`", b),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                },
                            ))
                        }
                    }
                }
                TokenType::BlokSonlandır => {
                    let mut map = Map::new();
                    if let Some(last_ele) = stack.pop() {
                        stack.push_ret(last_ele);
                    }
                    stack.del_stack();
                    let fhash = hashs.del_hash().unwrap();
                    for (k, v) in fhash.into_iter() {
                        map.map.insert(k, v);
                    }
                    hashs.insert(current_namespace.pop().unwrap(), Object::Harita(map));
                    self.current += 1;
                }
                TokenType::Blok => {
                    stack.new_stack();
                    hashs.new_hash();
                    let id = self.program.get(self.current + 1).unwrap();
                    match id.typ.clone() {
                        TokenType::Identifier { id: ident } => {
                            current_namespace.push(ident);
                        }
                        _ => {
                            return Err((
                                stack,
                                hashs,
                                match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BeklenmedikSimge",
                                        &format!(
                                            "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                            id.repr()
                                        ),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BeklenmedikSimge",
                                        &format!("expected identifier, but found `{}`", id.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                },
                            ))
                        }
                    }
                    self.current += 2;
                }
                TokenType::Hiç => {
                    stack.push(Object::Hiç);
                    self.current += 1;
                }
                TokenType::ParenL => unreachable!(),
                TokenType::Ver { tp } => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                            "KümedeYeterliDeğişkenYok",
                            &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                            None,
                        )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                            "KümedeYeterliDeğişkenYok",
                            &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                            None,
                        )
                            }
                        })),
                    };
                    stack.push_ret(a);
                    if let Some(i) = tp {
                        match self.program.get_mut(i).unwrap().typ {
                            TokenType::İşlevSonlandır { ref mut tp } => {
                                if let Some(u) = tp.pop() {
                                    self.current = u;
                                }
                            }
                            _ => unreachable!(),
                        };
                        self.current += 1;
                        traceback.pop();
                        stack.del_stack();
                        hashs.del_hash();
                    } // error
                }
                TokenType::İşlev { sonloc } => {
                    let id = self.program.get(self.current + 1).unwrap();
                    match id.typ.clone() {
                        TokenType::Identifier { id: ident } => {
                            hashs.insert(ident, Object::İşlev(self.current));
                        }
                        _ => {
                            return Err((
                                stack,
                                hashs,
                                match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BeklenmedikSimge",
                                        &format!(
                                            "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                            id.repr()
                                        ),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BeklenmedikSimge",
                                        &format!("expected identifier, but found `{}`", id.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                },
                            ))
                        }
                    }
                    let loc = match sonloc {
                        Some(a) => a,
                        None => unreachable!(),
                    };
                    match self.program.get_mut(loc).unwrap().typ {
                        TokenType::İşlevSonlandır { ref mut tp } => {
                            tp.push(loc);
                        }
                        _ => unreachable!(),
                    }
                    self.current = loc + 1;
                }
                TokenType::İşlevSonlandır { .. } => {
                    let loc = match token.typ {
                        TokenType::İşlevSonlandır { ref mut tp } => tp.pop().unwrap(), // Safe to unwrap
                        _ => unreachable!(),
                    };
                    self.current = loc;
                    self.current += 1;
                    traceback.pop();
                    stack.del_stack();
                    hashs.del_hash();
                }
                TokenType::De => {
                    print!(
                        "{:?}",
                        match stack.pop() {
                            Some(a) => a,
                            None =>
                                return Err((
                                    stack,
                                    hashs,
                                    match get_lang() {
                                        SupportedLanguage::Turkish => {
                                            Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                                        }
                                        SupportedLanguage::English => {
                                            Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                                        }
                                    }
                                )),
                        }
                    );
                    self.current += 1;
                }
                TokenType::Artı => {
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                            }
                        })),
                    };
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                "KümedeYeterliDeğişkenYok",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                None,
                            )
                            }
                        })),
                    };
                    stack.push(match a.ekle(b) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::ArtıArtı => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match a.ekle(Object::Sayı(1.0)) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Eksi => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.çıkar(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::EksiEksi => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match a.çıkar(Object::Sayı(1.0)) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Çarpı => {
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match a.çarp(b) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Bölü => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.böl(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Sayı { val } => {
                    let n = Object::Sayı(val);
                    stack.push(n);
                    self.current += 1;
                }
                TokenType::Yazı { val } => {
                    let s = Object::Yazı(val);
                    stack.push(s);
                    self.current += 1;
                }
                TokenType::Bool { val } => {
                    let b = Object::Bool(val);
                    stack.push(b);
                    self.current += 1;
                }
                TokenType::İse(yoksa) | TokenType::İken(yoksa) => {
                    if let Some(tp) = yoksa {
                        let a = match stack.pop() {
                            Some(a) => a,
                            None => return Err((stack, hashs, match get_lang() {
                                SupportedLanguage::Turkish => {
                                    Error::new(
                                            "KümedeYeterliDeğişkenYok",
                                            &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                            None,
                                        )
                                }
                                SupportedLanguage::English => {
                                    Error::new(
                                            "KümedeYeterliDeğişkenYok",
                                            &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                            None,
                                        )
                                }
                            })),
                        };
                        match a {
                            Object::Bool(b) => {
                                if b {
                                    self.current += 1;
                                } else {
                                    self.current = tp;
                                }
                            }
                            _ => {
                                let b = match stack.pop() {
                                    Some(a) => a,
                                    None => return Err((stack, hashs, match get_lang() {
                                        SupportedLanguage::Turkish => {
                                            Error::new(
                                                    "KümedeYeterliDeğişkenYok",
                                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                    None,
                                                )
                                        }
                                        SupportedLanguage::English => {
                                            Error::new(
                                                    "KümedeYeterliDeğişkenYok",
                                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                                    None,
                                                )
                                        }
                                    })),
                                };
                                match b.eşittir(a) {
                                    Ok(Object::Bool(true)) => self.current += 1,
                                    Ok(Object::Bool(false)) => self.current = tp,
                                    Ok(_) => unreachable!(),
                                    Err(e) => return Err((stack, hashs, e)),
                                }
                            }
                        }
                    } else {
                        unreachable!()
                    }
                }
                TokenType::Sına  => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                        "KümedeYeterliDeğişkenYok",
                                        &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                        "KümedeYeterliDeğişkenYok",
                                        &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    )
                            }
                        })),
                    };
                    match a {
                        Object::Bool(b) => {
                            if b {
                                self.current += 1;
                            } else {
                                return Err((stack, hashs, match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BaşarısızSınama",
                                        "Sınama başarısız oldu",
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BaşarısızSınama",
                                        "Test was unsuccessful",
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                }));
                            }
                        }
                        b => return Err((
                             stack,
                             hashs,
                             match get_lang() {
                                 SupportedLanguage::Turkish => Error::new(
                                     "BeklenmedikTip",
                                     &format!("bool beklenmişti ancak `{:?}` bulundu", b),
                                     { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                     None,
                                 ),
                                 SupportedLanguage::English => Error::new(
                                     "BeklenmedikTip",
                                     &format!("expected bool but found `{:?}`", b),
                                     { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                     None,
                                 ),
                             },
                        )),
                    }
                }
                TokenType::Kopya => {
                    let last = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(last.clone());
                    stack.push(last);
                    self.current += 1;
                }
                TokenType::Büyüktür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.büyüktür(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::BüyükEşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.büyük_eşittir(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Küçüktür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.küçüktür(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::KüçükEşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.küçük_eşittir(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Eşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.eşittir(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::EşitDeğildir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.eşit_değildir(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Değildir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match a.değildir() {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Son { tp } => {
                    self.current = tp;
                }
                TokenType::Yoksa(yoksa) => {
                    if let Some(tp) = yoksa {
                        self.current = tp;
                    } else {
                        unreachable!()
                    }
                }
                TokenType::Modulo => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(match b.modulo(a) {
                        Ok(a) => a,
                        Err(e) => return Err((stack, hashs, e)),
                    });
                    self.current += 1;
                }
                TokenType::Takas => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(a);
                    stack.push(b);
                    self.current += 1;
                }
                TokenType::Döndür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let c = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(a);
                    stack.push(b);
                    stack.push(c);
                    self.current += 1;
                }
                TokenType::At => {
                    match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    self.current += 1;
                }
                TokenType::Üst => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(b.clone());
                    stack.push(a);
                    stack.push(b);
                    self.current += 1;
                }
                TokenType::Girdi => {
                    io::stdout().flush().unwrap();
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();
                    stack.push(Object::Yazı(buf.trim_end().to_string()));
                    self.current += 1;
                }
                TokenType::İkiNoktaNokta | TokenType::EOF => self.current += 1,
                TokenType::Identifier { id } => match hashs.clone().get_mut(&id) {
                    Some(val) => match val {
                        Object::Hiç
                        | Object::Bool(_)
                        | Object::Sayı(_)
                        | Object::Yazı(_)
                        | Object::Liste(_)
                        | Object::FfiObject(_)
                        | Object::Harita(_) => {
                            stack.push(val.clone());
                            self.current += 1;
                        }
                        Object::FfiFunction(f) => {
                            let res = f.call(&mut stack, &mut hashs);
                            match res {
                                Ok(Some(o)) => stack.push(o),
                                Ok(_) => (),
                                Err(e) => return Err((stack, hashs, e)),
                            }
                            self.current += 1;
                        }
                        Object::İşlev(tp) => {
                            let işlev = self.program.get(*tp).unwrap();
                            match işlev.typ {
                                TokenType::İşlev { sonloc: tpi } => {
                                    let loc = match tpi {
                                        Some(i) => i,
                                        None => unreachable!(),
                                    };
                                    let işlevson = self.program.get_mut(loc).unwrap();
                                    match &mut işlevson.typ {
                                        TokenType::İşlevSonlandır { tp: ref mut tps } => {
                                            tps.push(self.current);
                                        }
                                        _ => unreachable!(),
                                    }
                                    self.current = *tp + 2;
                                }
                                _ => unreachable!(),
                            }
                            
                            if let TokenType::Identifier { id : fname } =
                                &self.program.get(*tp + 1).unwrap().typ {
                                traceback.push((tokenc.line, tokenc.col, tokenc.file, Some(fname.clone())));
                            }
                            stack.new_stack();
                            hashs.new_hash();
                        }
                    },
                    None => {
                        return Err((stack, hashs.clone(), match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "BilinmeyenTanımlayıcı",
                                    &format!(
                                        "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                                        tokenc.repr()
                                    ),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    {
                                        let mut hashk = hashs.clone().into_keys();
                                        hashk.sort();
                                        let n = hashk.binary_search(&id).unwrap_err();
                                        if hashk.is_empty() {
                                            None
                                        } else {
                                            Some(format!("`{}` demek mi istediniz?", hashk[n]))
                                        }
                                    }
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                        "BilinmeyenTanımlayıcı",
                                        &format!("unknown identifier: `{}`, this identifier could not be found", tokenc.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        {
                                            let mut hashk = hashs.clone().into_keys();
                                            hashk.sort();
                                            let n = hashk.binary_search(&id).unwrap_err();
                                            if hashk.is_empty() {
                                                None
                                            } else {
                                                Some(format!("maybe you meant {}?", hashk[n]))
                                            }
                                        },
                                    )
                            }
                        }));
                    }
                },
                TokenType::Koy => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let id = self.program.get(self.current + 1).unwrap();
                    hashs.insert(
                        match id.typ.clone() {
                            TokenType::Identifier { id: i } => i,
                            t => {
                                return Err((
                                    stack,
                                    hashs,
                                    match get_lang() {
                                        SupportedLanguage::Turkish => Error::new(
                                            "BeklenmedikSimge",
                                            &format!(
                                            "Tanımlayıcı simgesi beklenmişti ancak {:?} bulundu",
                                            t
                                        ),
                                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                            None,
                                        ),
                                        SupportedLanguage::English => Error::new(
                                            "BeklenmedikSimge",
                                            &format!("expected Identifier but found {:?}", t),
                                            { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                            None,
                                        ),
                                    },
                                ))
                            }
                        },
                        a,
                    );
                    self.current += 2;
                }
                TokenType::Ve => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(
                        match b.ve(a, tokenc.line, tokenc.col, tokenc.file.clone()) {
                            Ok(a) => a,
                            Err(e) => return Err((stack, hashs, e)),
                        },
                    );
                    self.current += 1;
                }
                TokenType::Veya => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    stack.push(
                        match b.veya(a, tokenc.line, tokenc.col, tokenc.file.clone()) {
                            Ok(a) => a,
                            Err(e) => return Err((stack, hashs, e)),
                        },
                    );
                    self.current += 1;
                }
                TokenType::Tipinde => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err((stack, hashs, match get_lang() {
                            SupportedLanguage::Turkish => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                Error::new(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                    None,
                                )
                            }
                        })),
                    };
                    self.current += 1;
                    let b = self.program.get_mut(self.current).unwrap();
                    match &b.typ {
                        TokenType::Identifier { id } => {
                            stack.push(
                                match a.dönüştür(id.clone(), b.line, b.col, b.file.clone()) {
                                    Ok(a) => a,
                                    Err(e) => return Err((stack, hashs, e)),
                                },
                            );
                        }
                        _ => {
                            return Err((
                                stack,
                                hashs,
                                match get_lang() {
                                    SupportedLanguage::Turkish => Error::new(
                                        "BeklenmedikSimge",
                                        &format!(
                                            "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                            b.repr()
                                        ),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                    SupportedLanguage::English => Error::new(
                                        "BeklenmedikSimge",
                                        &format!("expected identifier, but found `{}`", b.repr()),
                                        { traceback.push((tokenc.line, tokenc.col, tokenc.file, None)); traceback },
                                        None,
                                    ),
                                },
                            ))
                        }
                    };
                    self.current += 1;
                }
            }
        }

        let mut edt = Ok(());
        for i in loaded_libraries.into_iter() {
            if let Err(e) = terminate_library(&i.1, i.0, &mut stack, &mut hashs) {
                if let Ok(()) = edt {
                    edt = Err(e);
                }
            }
        }
        if let Err(e) = edt {
            return Err((stack, hashs, Error::new(
                "DinamikYüklemeHatası",
                &format!("{}", e),
                traceback,
                None,
            )));
        }

        if stack.len() > 0 && !config.supress_warnings && !config.repl {
            match get_lang() {
                SupportedLanguage::Turkish => {
                    Error::warning(
                        "KümeBoşDeğil",
                        "küme boş değil, eğer nedeninden şüphe ediyorsanız kodunuzu kontrol etmeniz önerilir",
                        { traceback.push((0, 0, file, None)); traceback },
                        None
                    );
                    print!("    kümede kalan değişkenler({:?}) [", stack.len());
                    for (i, o) in stack.iter_vec().iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Hiç
                            | Object::Bool(_)
                            | Object::Sayı(_)
                            | Object::Liste(_)
                            | Object::Harita(_) => format!("{:?}", o),
                            Object::FfiObject(o) => o.repr(),
                            Object::İşlev(_) | Object::FfiFunction(_) => unreachable!(),
                        };
                        if i > 0 {
                            print!(", {}", o);
                        } else {
                            if stack.len() > 3 {
                                print!("... {}", o);
                            } else {
                                print!("{}", o);
                            }
                        }
                    }
                    println!("]");
                }
                SupportedLanguage::English => {
                    Error::warning(
                        "StackNotEmpty",
                        "stack is not empty, if you aren't sure about why, you might want to take a look at you code",
                        { traceback.push((0, 0, file, None)); traceback },
                        None
                    ).warn();
                    print!("    variables left in the stack({:?}) [", stack.len());
                    for (i, o) in stack.iter_vec().iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Hiç
                            | Object::Bool(_)
                            | Object::Sayı(_)
                            | Object::Liste(_)
                            | Object::Harita(_) => format!("{:?}", o),
                            Object::FfiObject(o) => o.repr(),
                            Object::İşlev(_) | Object::FfiFunction(_) => unreachable!(),
                        };
                        if i > 0 {
                            print!(", {}", o);
                        } else {
                            if stack.len() > 3 {
                                print!("...{}", o);
                            } else {
                                print!("{}", o);
                            }
                        }
                    }
                    println!("]");
                }
            }
        }
        Ok((stack, hashs))
    }
}
