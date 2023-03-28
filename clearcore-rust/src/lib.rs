/*!
 *  Rust wrapper for the ClearCore I/O and Motion library provided by Teknic, Inc.
 * 
 *  Author: Mike Werezak
 */

#![no_std]

pub mod connector;
pub mod timing;

/// Raw bindings.
pub mod bindings;

// Panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

