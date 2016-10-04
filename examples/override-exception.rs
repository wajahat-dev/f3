//! Override the hard fault (or any other) exception handler

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate f3;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    let _hard_fault_exception = unsafe {
        // After you hit this exception ...
        *((0x4000_0000 + 40 * 1024) as *const u32)
    };

    loop {}
}

#[export_name = "_hard_fault"]  // <-- Important! Note the underscore.
pub extern "C" fn my_hard_fault_handler() {
    unsafe {
        // .. you should reach THIS breakpoint!
        bkpt!();
    }

    loop {}
}