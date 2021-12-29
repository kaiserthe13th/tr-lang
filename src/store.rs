use cfg_if::cfg_if;

pub mod globarg {
    pub static mut SUPRESS_WARN: bool = false;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const LICENSE: &str = include_str!("../LICENSE");
pub const RELEASE: &str = env!("RELEASE_DATE");

cfg_if! {
    if #[cfg(windows)] {
        pub const PATH_SEP: char = '\\';
    } else {
        pub const PATH_SEP: char = '/';
    }
}
