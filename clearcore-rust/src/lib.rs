//! Rust wrapper for the ClearCore I/O and Motion library provided by Teknic, Inc.

#![no_std]

pub mod bindings;

// Panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

