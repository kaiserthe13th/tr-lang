#![feature(io_error_more)]

//! tr-lang programming language is a language that aims to make programming simpler for
//! turkish speaking groups

pub mod lexer;
pub use lexer::Lexer;

pub mod parser;
pub use parser::Parser;

pub mod bytecode;
pub mod error;
pub mod mem;
pub(crate) mod store;
pub mod token;
pub(crate) mod util;
#[cfg(feature = "fmt")]
pub mod fmt;

pub mod runtime;
pub use runtime::Run;
#[cfg(feature = "interactive")]
pub mod interactive;
