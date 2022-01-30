use cfg_if::cfg_if;

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
