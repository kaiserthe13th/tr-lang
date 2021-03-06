use std::{error::Error, fs, path::{Path, PathBuf}};
use tr_lang::{
    error::Error as TrlError,
    mem::{HashMemory, StackMemory},
    Lexer, Parser, Run,
    runtime::RunConfig,
};
use wax::Glob;

pub enum TestError {
    Failed(StackMemory, HashMemory, TrlError),
    FSError(Box<dyn Error>),
    PreRuntimeError(TrlError),
}

pub fn test_file(path: &Path) -> Result<Option<(StackMemory, HashMemory)>, TestError> {
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
                    let r = run.run(RunConfig {
                        file: path.display().to_string(),
                        supress_warnings: true,
                        ..Default::default()
                    });
                    match r {
                        Ok(mem) => Ok(Some(mem)),
                        Err((s, h, e)) => Err(TestError::Failed(s, h, e)),
                    }
                }
                Err(ref _e) => parsed.map(|_| None).map_err(TestError::PreRuntimeError),
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

