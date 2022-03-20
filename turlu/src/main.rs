#![feature(result_flattening)]
#![feature(is_some_with)]

mod tester;
use same_file::is_same_file;
use tester::TestError;
use tr_lang::mem::Map;
use tr_lang::mem::Object;

use std::process::exit;
use std::collections::HashMap;
use std::error::Error;

use bunt;

use serde::Deserialize;
use serde_yaml as yml;
use serde_yaml::Value;

use std::fs;
use std::env;

#[derive(Deserialize, Debug)]
struct VarMap {
    #[serde(rename = "ad-alanı")]
    ad_alanı: Option<HashMap<String, Value>>,
    stack: Option<Vec<Value>>,
}

trait IntoObject {
    fn into_object(self) -> Object;
}

impl IntoObject for Value {
    fn into_object(self) -> Object {
        match self {
            Value::Null => Object::Hiç,
            Value::Bool(b) => Object::Bool(b),
            Value::Number(n) => Object::Sayı(n.as_f64().unwrap()),
            Value::String(s) => Object::Yazı(s),
            Value::Mapping(m) => {
                let mut h = Map::new();
                for (k, v) in m.into_iter() {
                    h.map.insert(k.as_str().unwrap().to_string(), v.into_object());
                }
                Object::Harita(h)
            },
            Value::Sequence(_v) => todo!(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    dil: Option<String>,
    testler: Vec<String>,
    bekle: Option<HashMap<String, Vec<String>>>,
    varmap: Option<VarMap>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ecode = 0;

    let config: Config = yml::from_str(&fs::read_to_string("turlu.yml")?)?;
    env::set_var("LANG", config.dil.unwrap_or(env::var("LANG").unwrap_or("en_GB.UTF-8".to_string())));
    bunt::println!("{$blue+bold}testler aranıyor...{/$}");
    let tests = tester::find_tests(config.testler).expect("testler bulunamadı");
    bunt::println!("{$white}{[blue]:?} bulundu{/$}", tests.iter().map(|a|a.display()).collect::<Vec<_>>());
    for test in tests.iter() {
        bunt::println!("\n{$white}{[blue+bold]:?} yürütülüyor{/$}", test.display());
        if let Some(path) = config.bekle.as_ref()
            .map(|a| a.keys()
                 .find_map(|t| is_same_file(&test, t).map(|b| if b { Some(t.to_string()) } else { None }).ok())
            ).flatten().flatten() {
            let merrs: Vec<String> = config.bekle.as_ref().unwrap().get(&path).unwrap().to_vec();
            match tester::test_file(test) {
                Ok(_) => {
                    bunt::eprintln!(
                        "\n{:?} dan biri bekleniyordu ancak bulunamadı",
                        merrs,
                    );
                    bunt::eprintln!("{$red+bold}başarısız{/$}");
                    ecode = 1;
                }
                Err(TestError::Failed(sm, hm, err)) => {
                    if merrs.into_iter().any(|a| a == err.name()) {
                        bunt::println!("\n{$green+bold}başarılı{/$} ({})", err.name());
                    } else {
                        err.error_print();
                        bunt::eprintln!("{$#b268e3+italic+bold}Stack Durumu:{/$}");
                        eprintln!("{:#?}", sm);
                        bunt::eprintln!("{$#b268e3+italic+bold}Ad Alanı Durumu:{/$}");
                        eprintln!("{:#?}", hm);
                        bunt::eprintln!("{$red+bold}başarısız{/$}");
                        ecode = 1;
                    }
                },
                Err(TestError::FSError(e)) => return Err(e),
                Err(TestError::PreRuntimeError(e)) => {
                    if merrs.into_iter().any(|a| a == e.name()) {
                        bunt::print!("\n({})", e.name());
                        bunt::println!("{$green+bold}başarılı{/$}");
                    } else {
                        e.error_print();
                        bunt::eprintln!("{$red+bold}başarısız{/$}");
                        ecode = 1;
                    }
                },
            }
        } else {
            match tester::test_file(test) {
                Ok(v) => {
                    if let Some(vm) = &config.varmap {
                        for (mut sm, mut hm) in v {
                            let mut stat = if let Some(s) = &vm.stack {
                                s.iter()
                                    .map(|a| a.clone().into_object())
                                    .zip(sm.iter_vec().iter())
                                    .all(|(a, b)| a.eşittir(b.clone())
                                        .map(|o| match o {
                                            Object::Bool(b) => b,
                                            _ => unreachable!(),
                                        })
                                        .map_err(|_| false)
                                        .unwrap_or_else(|a|a)
                                    )
                            } else { true };
                            stat = stat && if let Some(h) = &vm.ad_alanı {
                                h.iter().all(|(k,v)|
                                    if let Some(w) = hm.get(k) {
                                        v.clone().into_object().eşittir(w.clone())
                                            .map(|o| match o {
                                                Object::Bool(b) => b,
                                                _ => unreachable!(),
                                            })
                                            .map_err(|_| false)
                                            .unwrap_or_else(|a|a)
                                    } else {
                                        false
                                    }
                                )
                            } else { true };
                            if stat {
                                bunt::println!("\n{$green+bold}başarılı{/$}");
                            } else {
                                bunt::eprintln!("{$red+bold}başarısız{/$}");
                            }
                        }
                    } else {
                        bunt::println!("\n{$green+bold}başarılı{/$}");
                    }
                }
                Err(TestError::Failed(sm, hm, err)) => {
                    err.error_print();
                    bunt::eprintln!("{$#b268e3+italic+bold}Stack Durumu:{/$}");
                    eprintln!("{:#?}", sm);
                    bunt::eprintln!("{$#b268e3+italic+bold}Ad Alanı Durumu:{/$}");
                    eprintln!("{:#?}", hm);
                    bunt::eprintln!("{$red+bold}başarısız{/$}");
                    ecode = 1;
                },
                Err(TestError::FSError(e)) => return Err(e),
                Err(TestError::PreRuntimeError(e)) => {
                    e.error_print();
                    bunt::eprintln!("{$red+bold}başarısız{/$}");
                    ecode = 1;
                },
            }
        }
    }
    if ecode != 0 {
        exit(ecode);
    }
    Ok(())
}

