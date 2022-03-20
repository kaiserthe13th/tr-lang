use std::{error::Error, fs, path::{Path, PathBuf}};
use tr_lang::{
    error::Error as TrlError,
    mem::{HashMemory, Object, StackMemory},
    Lexer, Parser, Run,
    token::{
        ParserToken,
        tokentypes::ParserTokenType,
    },
};
use wax::Glob;

pub enum TestError {
    Failed(StackMemory, HashMemory, TrlError),
    FSError(Box<dyn Error>),
    PreRuntimeError(TrlError),
}

pub fn test_file(path: &Path) -> Result<Vec<(StackMemory, HashMemory)>, TestError> {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(ctx) => {
            let mut lexer = Lexer::new(ctx);
            let parsed = Parser::from_lexer(&mut lexer, path.display().to_string())
                .map(|mut a| a.parse())
                .flatten();
            match parsed {
                Ok(psd) => {
                    let mut run = Run::new(psd.clone());
                    let r = run.run(path.display().to_string(), None, false);
                    match r {
                        Ok((_stm, mut hsm)) => {
                            let mut test_fns = vec![];

                            let hs = hsm.clone().into_keys();
                            let varnames = hs.iter().filter(|a| a.starts_with("test-"));
                            for var in varnames {
                                let o = hsm.get(&var).unwrap();
                                match o {
                                    Object::İşlev(pos) => {
                                        test_fns.push(*pos);
                                    }
                                    _ => (),
                                }
                            }
                            let mut mems = vec![];
                            for test in test_fns {
                                let mut pt = psd.clone();
                                pt.push(ParserToken::new(
                                    ParserTokenType::Identifier { id: match &psd.get(test + 1).unwrap().typ {
                                        ParserTokenType::Identifier { id } => id.clone(),
                                        _ => unreachable!(), // Should Error out before this
                                    }},
                                    0, 0, "<test>".to_string(),
                                ));
                                let mut runt = Run::new(pt);
                                mems.push(runt.run(path.display().to_string(), None, false)
                                    .map_err(|(s, h, e)| TestError::Failed(s, h, e))?);
                            }
                            Ok(mems)
                        }
                        Err((s, h, e)) => Err(TestError::Failed(s, h, e)),
                    }
                }
                Err(ref _e) => parsed.map(|_| vec![]).map_err(TestError::PreRuntimeError),
            }
        }
        Err(e) => Err(TestError::FSError(Box::new(e))),
    }
}

pub fn find_tests(matchers: Vec<String>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut res = vec![];
    for matcher in matchers {
        for i in Glob::new(&matcher).expect("eşleyici oluşturulamadı").walk(".", usize::MAX) {
            res.push(i.unwrap().path().to_path_buf());
        }
    }
    Ok(res)
}

