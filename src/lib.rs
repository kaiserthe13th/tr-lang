#![feature(io_error_more)]
#![feature(associated_type_defaults)]

//! tr-lang programming language is a language that aims to make programming simpler for
//! turkish speaking groups

pub mod lexer;
pub mod parser;
pub mod bytecode;
pub mod error;
pub mod mem;
pub(crate) mod store;
pub mod token;
pub(crate) mod util;
#[cfg(feature = "fmt")]
pub mod fmt;
pub mod runtime;
#[cfg(feature = "interactive")]
pub mod interactive;
pub mod ffi;

pub mod prelude {
    use crate::*;

    pub use lexer::Lexer;
    pub use parser::Parser;
    pub use runtime::Run;
    pub use crate::ffi as tffi;
}

#[macro_export]
macro_rules! run {
    ($fl:expr; $($x:tt)*) => {
        let mut buf = String::new();
        $(
            buf.push_str(stringify!($x));
            buf.push(' ');
        )*
        let p = $crate::parser::Parser::from_lexer(&mut $crate::lexer::Lexer::new(buf), $fl)
            .map(|mut a| a.parse().map(|b| $crate::runtime::Run::new(b).run($crate::runtime::RunConfig::default())));
    };
    ($fl:ident; $($x:tt)*) => {
        let mut buf = String::new();
        $(
            buf.push_str(stringify!($x));
            buf.push(' ');
        )*
        let p = $crate::parser::Parser::from_lexer(&mut $crate::lexer::Lexer::new(buf), stringify!($fl).to_string())
            .map(|mut a| a.parse().map(|b| $crate::runtime::Run::new(b).run($crate::runtime::RunConfig::default())));
    };
    ($($x:tt)*) => {
        let mut buf = String::new();
        $(
            buf.push_str(stringify!($x));
            buf.push(' ');
        )*
        let p = $crate::parser::Parser::from_lexer(&mut $crate::lexer::Lexer::new(buf), "rust".to_string())
            .map(|mut a| a.parse().map(|b| $crate::runtime::Run::new(b).run($crate::runtime::RunConfig::default())));
    };
}

