use wasm_bindgen::{prelude::wasm_bindgen, convert::{WasmPrimitive, WasmAbi}};

mod bootloader;
use bootloader::haptloader;

trait Primitive<T>: WasmPrimitive + WasmAbi {
    fn get(self) -> T;
    fn set(self, value: usize) -> T;
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Class(i64);

unsafe impl WasmPrimitive for Class {}

impl Primitive<i64> for Class {
    fn get(self) -> i64 {
        unsafe { 
            let store = haptloader::LOCAL_STORAGE.get_mut(self.0 as usize).expect("[hapt-storage] failed to get from local_storage");
            std::mem::transmute((store.0 as u32, store.1 as u32))
        }
    }

    fn set(self, value: usize) -> i64 {
        unsafe {
            let store = haptloader::LOCAL_STORAGE.get_mut(self.0 as usize).expect("[hapt-storage] failed to get from local_storage");
            *store = std::mem::transmute(value as u16);
            std::mem::transmute((store.0 as u32, store.1 as u32))
        }
    }
}

#[wasm_bindgen]
pub fn init() -> i64 {
    let ehpt = bootloader::init().expect("[meluloader] help");
    unsafe { std::mem::transmute((ehpt.0 as u32, ehpt.1 as u32)) }
}

#[wasm_bindgen]
pub fn get(class: i64) -> i64 {
    Class(class).get()
}

#[wasm_bindgen]
pub fn set(class: i64, value: usize) -> i64 {
    Class(class).set(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
