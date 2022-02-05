#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod globarg {
    pub static mut SUPRESS_WARN: bool = false;
}
