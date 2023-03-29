#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::ffi;

use clearcore;
use clearcore::timing::delay_ms;


// main function - symbol should match the one declared in `Device_Startup/startup_same53.c`
#[no_mangle]
pub extern "C" fn main() -> ffi::c_int {
    let mut led_state = true;
    let mut led = clearcore::connector_led();
    loop {
        led.set_state(led_state);
        led_state = !led_state;
        delay_ms(1000);
    }
}
