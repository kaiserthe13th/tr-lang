use cfg_if::cfg_if;

pub mod globarg {
    pub static mut SUPRESS_WARN: bool = false;
}
pub const VERSION: &str = "0.3.1";
pub const RELEASE: &str = "16.11.2021";
cfg_if! {
    if #[cfg(windows)] {
        pub const PATH_SEP: char = '\\';
    } else {
        pub const PATH_SEP: char = '/';
    }
}