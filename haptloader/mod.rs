pub mod bootloader;
pub mod header;

pub fn init() -> Result<(usize, usize), ()> {
    header::init(); bootloader::init()
}