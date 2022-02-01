use crate::errwarn::ErrorGenerator;
use crate::mem::{HashMemory, Object, StackMemory};
use crate::store::globarg::SUPRESS_WARN;
use crate::token::{tokentypes::ParserTokenType as TokenType, ParserToken as Token};
use crate::util::{get_lang, SupportedLanguage};
use std::io::{self, prelude::*};
use crate::errwarn::Error;

pub struct Run {
    program: Vec<Token>,
    current: usize,
}

impl Run {
    pub fn new(program: Vec<Token>) -> Self {
        Self {
            program,
            current: 0,
        }
    }

    pub fn run(&mut self, file: String) -> Result<(), Error> {
        let mut stack = StackMemory::new();
        let mut hashs = HashMemory::new();
        // let mut warnings: Vec<Box<dyn FnOnce()>> = vec![]; // for later use

        while self.program.len() > self.current {
            let tokenc = self.program.get(self.current).unwrap().clone();
            let token = self.program.get_mut(self.current).unwrap();

            match token.typ.clone() {
                TokenType::ParenL => unreachable!(),
                TokenType::Ver { tp } => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                            "KümedeYeterliDeğişkenYok",
                            &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                            tokenc.line,
                            tokenc.col,
                            tokenc.file,
                            None,
                        )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                            "NotEnoughVarsInStack",
                            &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                            tokenc.line,
                            tokenc.col,
                            tokenc.file,
                            None,
                        )
                            }
                        }),
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
                        _ => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BeklenmedikSimge",
                                    &format!(
                                        "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                        id.repr()
                                    ),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "UnexpectedToken",
                                    &format!("expected identifier, but found `{}`", id.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
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
                    stack.del_stack();
                    hashs.del_hash();
                }
                TokenType::De => {
                    print!(
                        "{:?}",
                        match stack.pop() {
                            Some(a) => a,
                            None => return Err(match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                "NotEnoughVarsInStack",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                                }
                            }),
                        }
                    );
                    io::stdout().flush().unwrap();
                    self.current += 1;
                }
                TokenType::Artı => {
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                "NotEnoughVarsInStack",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                            }
                        }),
                    };
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                "KümedeYeterliDeğişkenYok",
                                &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                "NotEnoughVarsInStack",
                                &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                tokenc.line,
                                tokenc.col,
                                tokenc.file,
                                None,
                            )
                            }
                        }),
                    };
                    stack.push(match a.ekle(b) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::ArtıArtı => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match a.ekle(Object::Sayı(1.0)) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Eksi => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.çıkar(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::EksiEksi => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match a.çıkar(Object::Sayı(1.0)) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Çarpı => {
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match a.çarp(b) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Bölü => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.böl(a) { Ok(a) => a, Err(e) => return Err(e) });
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
                            None => return Err(match get_lang() {
                                SupportedLanguage::Turkish => {
                                    ErrorGenerator::error(
                                            "KümedeYeterliDeğişkenYok",
                                            &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                            tokenc.line,
                                            tokenc.col,
                                            tokenc.file,
                                            None,
                                        )
                                }
                                SupportedLanguage::English => {
                                    ErrorGenerator::error(
                                            "NotEnoughVarsInStack",
                                            &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                            tokenc.line,
                                            tokenc.col,
                                            tokenc.file,
                                            None,
                                        )
                                }
                            }),
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
                                    None => return Err(match get_lang() {
                                        SupportedLanguage::Turkish => {
                                            ErrorGenerator::error(
                                                    "KümedeYeterliDeğişkenYok",
                                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                                    tokenc.line,
                                                    tokenc.col,
                                                    tokenc.file,
                                                    None,
                                                )
                                        }
                                        SupportedLanguage::English => {
                                            ErrorGenerator::error(
                                                    "NotEnoughVarsInStack",
                                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                                    tokenc.line,
                                                    tokenc.col,
                                                    tokenc.file,
                                                    None,
                                                )
                                        }
                                    }),
                                };
                                match b.eşittir(a) {
                                    Ok(Object::Bool(true)) => self.current += 1,
                                    Ok(Object::Bool(false)) => self.current = tp,
                                    Ok(_) => unreachable!(),
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                    } else {
                        unreachable!()
                    }
                }
                TokenType::Kopya => {
                    let last = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(last.clone());
                    stack.push(last);
                    self.current += 1;
                }
                TokenType::Büyüktür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.büyüktür(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::BüyükEşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.büyük_eşittir(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Küçüktür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.küçüktür(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::KüçükEşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.küçük_eşittir(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Eşittir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.eşittir(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::EşitDeğildir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.eşit_değildir(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Değildir => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match a.değildir() { Ok(a) => a, Err(e) => return Err(e) });
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
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.modulo(a) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Takas => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(a);
                    stack.push(b);
                    self.current += 1;
                }
                TokenType::Döndür => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let c = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(a);
                    stack.push(b);
                    stack.push(c);
                    self.current += 1;
                }
                TokenType::At => {
                    match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    self.current += 1;
                }
                TokenType::Üst => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` anahtar kelimesi uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the keyword `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(b.clone());
                    stack.push(a);
                    stack.push(b);
                    self.current += 1;
                }
                TokenType::Girdi => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();
                    stack.push(Object::Yazı(buf.trim_end().to_string()));
                    self.current += 1;
                }
                TokenType::İkiNoktaNokta | TokenType::EOF => self.current += 1,
                TokenType::Identifier { id } => match hashs.clone().get_mut(&id) {
                    Some(val) => match val {
                        Object::Bool(_)
                        | Object::Sayı(_)
                        | Object::Yazı(_)
                        | Object::Liste(_)
                        | Object::Harita(_) => {
                            stack.push(val.clone());
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
                            stack.new_stack();
                            hashs.new_hash();
                        }
                    },
                    None => {
                        return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BilinmeyenTanımlayıcı",
                                    &format!(
                                        "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                                        tokenc.repr()
                                    ),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    {
                                        let mut hashk = hashs.clone().into_keys();
                                        hashk.sort();
                                        let n = hashk.binary_search(&id).unwrap_err();
                                        if hashk.is_empty() {
                                            None
                                        } else {
                                            Some(format!("{}", hashk[n]))
                                        }
                                    }
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                        "UnknownIdentifier",
                                        &format!("unknown identifier: `{}`, this identifier could not be found", tokenc.repr()),
                                        tokenc.line,
                                        tokenc.col,
                                        tokenc.file,
                                        {
                                            let mut hashk = hashs.clone().into_keys();
                                            hashk.sort();
                                            let n = hashk.binary_search(&id).unwrap_err();
                                            if hashk.is_empty() {
                                                None
                                            } else {
                                                Some(format!("{}", hashk[n]))
                                            }
                                        },
                                    )
                            }
                        });
                    }
                },
                TokenType::Koy => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let id = self.program.get(self.current + 1).unwrap();
                    hashs.insert(match id.typ.clone() {
                        TokenType::Identifier { id : i } => i,
                        t => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BeklenmedikSimge",
                                    &format!("Tanımlayıcı simgesi beklenmişti ancak {:?} bulundu", t),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            },
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "UnexpectedToken",
                                    &format!("expected Identifier but found {:?}", t),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            },
                        }),
                    }, a);
                    self.current += 2;
                }
                TokenType::Ve => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.ve(a, tokenc.line, tokenc.col, tokenc.file.clone()) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Veya => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    let b = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    stack.push(match b.veya(a, tokenc.line, tokenc.col, tokenc.file.clone()) { Ok(a) => a, Err(e) => return Err(e) });
                    self.current += 1;
                }
                TokenType::Tipinde => {
                    let a = match stack.pop() {
                        Some(a) => a,
                        None => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "KümedeYeterliDeğişkenYok",
                                    &format!("kümede yeterli değişken bulunmadığından dolayı `{}` operatörü uygulanamamıştır", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "NotEnoughVarsInStack",
                                    &format!("because there weren't enough variables in the stack, the operator `{}` couldn't be used", tokenc.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    self.current += 1;
                    let b = self.program.get_mut(self.current).unwrap();
                    match &b.typ {
                        TokenType::Identifier { id } => {
                            stack.push(match a.dönüştür(id.clone(), b.line, b.col, b.file.clone()) { Ok(a) => a, Err(e) => return Err(e) });
                        }
                        _ => return Err(match get_lang() {
                            SupportedLanguage::Turkish => {
                                ErrorGenerator::error(
                                    "BeklenmedikSimge",
                                    &format!(
                                        "tanımlayıcı beklenmişti ancak `{}` bulundu",
                                        b.repr()
                                    ),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                            SupportedLanguage::English => {
                                ErrorGenerator::error(
                                    "UnexpectedToken",
                                    &format!("expected identifier, but found `{}`", b.repr()),
                                    tokenc.line,
                                    tokenc.col,
                                    tokenc.file,
                                    None,
                                )
                            }
                        }),
                    };
                    self.current += 1;
                }
            }
        }

        if stack.len() > 0 && !unsafe { SUPRESS_WARN } {
            match get_lang() {
                SupportedLanguage::Turkish => {
                    ErrorGenerator::warning(
                        "KümeBoşDeğil",
                        "küme boş değil, eğer nedeninden şüphe ediyorsanız kodunuzu kontrol etmeniz önerilir",
                        0,
                        0,
                        file
                    )();
                    print!("    kümede kalan değişkenler({:?}) [", stack.len());
                    for (i, o) in stack.iter_vec().iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Bool(_)
                            | Object::Sayı(_)
                            | Object::Liste(_)
                            | Object::Harita(_) => format!("{:?}", o),
                            Object::İşlev(_) => unreachable!(),
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
                    ErrorGenerator::warning(
                        "StackNotEmpty",
                        "stack is not empty, if you aren't sure about why, you might want to take a look at you code",
                        0,
                        0,
                        file
                    )();
                    print!("    variables left in the stack({:?}) [", stack.len());
                    for (i, o) in stack.iter_vec().iter().rev().take(3).rev().enumerate() {
                        let o = match o {
                            Object::Yazı(s) => format!("{:?}", s),
                            Object::Bool(_)
                            | Object::Sayı(_)
                            | Object::Liste(_)
                            | Object::Harita(_) => format!("{:?}", o),
                            Object::İşlev(_) => unreachable!(),
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
        Ok(())
    }
}
