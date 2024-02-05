#![no_std]
#![no_main]

//! This crate exits to take the same functions the c-interface would from
//! rs-interface, and try to call the code statically.
//! using the `no_panics_whatsoever` crate, we can statically guarantee that we never
//! introduce panics.
//!
//! This crate is never actually run
extern crate core;
extern crate rs_interface;
use rs_interface::*;

#[no_mangle]
extern "C" fn __libc_start_main() {
  core::hint::black_box({
    let _ = __rust_init();
    let _ = __rust_exit();
  })
}

#[no_mangle]
fn main() {}
