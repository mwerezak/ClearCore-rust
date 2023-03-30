#![no_std]
#![no_main]
#![allow(non_snake_case)]

use core::ffi;

use clearcore::{ConnectorLed};
use clearcore::connector::{LogicState, OutputDigital};
use clearcore::timing::delay_ms;


// main function - symbol should match the one declared in `Device_Startup/startup_same53.c`
#[no_mangle]
pub extern "C" fn main() -> ffi::c_int {
    let mut led = ConnectorLed::instance().unwrap();
    
    let mut led_state = LogicState::High;
    
    if let None = ConnectorLed::instance() {
        for _ in 1..10 {
            led.set_state(led_state);
            led_state = !led_state;
            delay_ms(100);
        }
    }
    
    loop {
        led.set_state(led_state);
        led_state = !led_state;
        delay_ms(1000);
    }
}
