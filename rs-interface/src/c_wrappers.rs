use crate::explicit_prelude::*;

use core::{ffi::*, ptr};
extern "C" {
  fn printk_(fmt: *const c_char, var_args: *const c_void);
}

/// Kernel print.
/// this can likely be made more safe
pub fn printk_str(message: &str) -> Result<(), &'static str> {
  if message.chars().any(|e| e == '%') {
    return Result::Err("Templating is not supported");
  };

  let bytes = message.as_bytes();
  if !bytes.iter().any(|e| e == &b'\0') {
    return Result::Err("strings must be null terminated");
  };

  // Safety: the contains a null, is ascii, and templating has been guarded against
  unsafe { printk_(bytes.as_ptr() as _, ptr::null()) }

  Result::Ok(())
}
