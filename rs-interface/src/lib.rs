#![no_std]
#![no_implicit_prelude]
pub(crate) mod explicit_prelude;

use explicit_prelude::*;
pub mod c_wrappers;

#[no_mangle]
pub extern "C" fn __rust_init() -> core::ffi::c_int {
  let _ = c_wrappers::printk_str("Hello Kernel\0");
  // core::panic!()
  0
}

#[no_mangle]
pub extern "C" fn __rust_exit() {
  let _ = c_wrappers::printk_str("Goodbye Kernel\0");
}
