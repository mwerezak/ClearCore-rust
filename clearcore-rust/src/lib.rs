/*!
 *  Rust wrapper for the ClearCore I/O and Motion library provided by Teknic, Inc.
 * 
 *  Author: Mike Werezak
 */

#![no_std]


pub(crate) mod bindings;

pub mod timing;
pub mod connector;


use core::sync::atomic::{AtomicBool, Ordering};
use connector::led_driver::LedDriver;

macro_rules! define_singletons {
    { $( $name:ident : $type:ty = $ctor:expr );* $(;)? } => {
        $(
            pub struct $name;

            impl $name {
                pub fn instance() -> Option<$type> {
                    static mut ONCE: AtomicBool = AtomicBool::new(false);
                    unsafe {
                        match ONCE.swap(false, Ordering::Acquire) {
                            true  => Some($ctor),
                            false => None,
                        }
                    }
                }
            }
        )*
    };
}


define_singletons! {

    /*
        Connectors
    */
    ConnectorLed: LedDriver = LedDriver::new(&mut bindings::ClearCore_ConnectorLed);

}


/*
    Panic Handler
*/
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

