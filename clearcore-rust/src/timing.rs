/*!
 *  ClearCore timing profiling utility functions.
 */

use crate::bindings;


#[inline]
pub fn delay_cycles(cycles: u64) {
    unsafe { bindings::Delay_cycles(cycles) }
}

#[inline]
pub fn delay_ms(ms: u32) {
    delay_cycles((ms as u64)*(bindings::CYCLES_PER_MILLISECOND as u64))
}

#[inline]
pub fn delay_us(us: u32) {
    delay_cycles((us as u64)*(bindings::CYCLES_PER_MICROSECOND as u64))
}
