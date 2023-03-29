/*!
 * 
 */

use crate::bindings;
use crate::connector::{Connector, refresh_connector};


/// Manages access to the LED shift register so LEDs may be controlled at the connector level.
pub struct LedDriver {
    ptr: *mut bindings::ClearCore_LedDriver,
}

impl LedDriver {
    pub(crate) fn new(ptr: *mut bindings::ClearCore_LedDriver) -> Self {
        LedDriver { ptr }
    }

    pub fn state(&self) -> bool {
        unsafe {
            bindings::ClearCore_LedDriver_State(self.ptr as *mut _) != 0
        }
    }

    pub fn set_state(&mut self, state: bool) {
        unsafe {
            bindings::ClearCore_LedDriver_State1(self.ptr as *mut _, state as _);
        }
    }

    unsafe fn connector_mut(&self) -> &mut bindings::ClearCore_Connector {
        &mut (*self.ptr)._base
    }
}

impl Connector for LedDriver {
    fn is_in_hw_fault(&self) -> bool {
        unsafe {
            bindings::ClearCore_LedDriver_IsInHwFault(self.ptr as *mut _)
        }
    }

    fn refresh(&mut self) {
        unsafe { refresh_connector(self.connector_mut()) }
    }

    fn reinitialize(&mut self) {
        unsafe {
            bindings::ClearCore_Connector_Reinitialize(self.ptr as *mut _)
        }
    }
}

