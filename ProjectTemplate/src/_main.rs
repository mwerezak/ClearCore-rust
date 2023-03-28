#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::ptr;
use core::ffi;
use clearcore::timing::delay_ms;

// TODO replace with wrapper code when available
use clearcore::bindings::{
    ClearCore_ConnectorLed, 
    ClearCore_LedDriver_State1,
};

// main function - symbol should match the one declared in `Device_Startup/startup_same53.c`
#[no_mangle]
pub extern "C" fn main() -> ffi::c_int {
    let mut led_state = true;
    loop {
        unsafe {
            let led_ptr = ptr::addr_of_mut!(ClearCore_ConnectorLed);
            ClearCore_LedDriver_State1(led_ptr as  *mut _, led_state as i16);

            led_state = !led_state;

            delay_ms(500);
        }
    }
}
