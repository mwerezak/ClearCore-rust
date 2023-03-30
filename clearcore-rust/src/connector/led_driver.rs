/*!
 * 
 */

use core::ptr::{NonNull, addr_of_mut};

use crate::bindings;
use crate::connector::ConnectorBase;
use crate::connector::modes::{OutputDigital, LogicState};


/// Manages access to the LED shift register so LEDs may be controlled at the connector level.
#[derive(Debug)]
pub struct LedDriver {
    ptr: NonNull<bindings::ClearCore_LedDriver>,
}

impl LedDriver {
    pub(crate) unsafe fn new(ptr: NonNull<bindings::ClearCore_LedDriver>) -> Self {
        LedDriver { ptr }
    }
}

// LedDriver is just always in output mode
impl OutputDigital for LedDriver {
    fn state(&self) -> LogicState {
        LogicState::from_raw(unsafe {
            bindings::ClearCore_LedDriver_State(self.ptr.as_ptr() as _)
        })
    }
    
    fn set_state(&mut self, state: LogicState) {
        unsafe {
            bindings::ClearCore_LedDriver_State1(self.ptr.as_ptr() as _, state.into_raw());
        }
    }
}

impl ConnectorBase for LedDriver {
    fn base_ptr(&self) -> NonNull<bindings::ClearCore_Connector> {
        unsafe {
            let base_ptr = addr_of_mut!((*self.ptr.as_ptr())._base);
            NonNull::new_unchecked(base_ptr)
        }
    }
}
