/*!
 *  Rust wrapper for the ClearCore I/O and Motion library provided by Teknic, Inc.
 * 
 *  Author: Mike Werezak
 */

#![no_std]

pub mod connector;
pub mod timing;

/// Raw bindings.
pub(crate) mod bindings;

use core::ptr::{addr_of_mut, NonNull};
use connector::led_driver::LedDriver;


/// Helper struct to ensure single access to static resources
// pub struct Singleton<T> {
//     instance: Option<T>,
// }

// impl<T> Singleton<T> {
//     fn new(instance: T) -> Self {
//         Self { instance: Some(instance) }
//     }
//     pub fn take(&mut self) -> Option<T> {
//         self.instance.take()
//     }
// }


/*
    Connectors
*/

// TODO make this a singleton some how?

/// User-driven LED instance.
pub fn connector_led() -> LedDriver {
    unsafe {
        LedDriver::new(NonNull::new_unchecked(addr_of_mut!(bindings::ClearCore_ConnectorLed)))
    }
}



// Panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

