use cfg_if::cfg_if;
#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod globarg {
    pub static mut SUPRESS_WARN: bool = false;
}

cfg_if! {
    if #[cfg(windows)] {
        pub const PATH_SEP: char = '\\';
    } else {
        pub const PATH_SEP: char = '/';
    }
}
