/*!
 * 
 */

use core::ptr::{NonNull, addr_of_mut};

use crate::bindings;
use crate::connector::Connector;
use crate::connector::modes::{OutputDigital, LogicState};


/// Manages access to the LED shift register so LEDs may be controlled at the connector level.
#[derive(Debug)]
pub struct LedDriver {
    ptr: NonNull<bindings::ClearCore_LedDriver>,
}

// Send but not Sync
unsafe impl Send for LedDriver { }

impl LedDriver {
    pub(crate) unsafe fn new(connector: &mut bindings::ClearCore_LedDriver) -> Self {
        LedDriver {
            ptr: NonNull::new_unchecked(addr_of_mut!(*connector))
        }
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

impl Connector for LedDriver {
    fn struct_ptr(&self) -> NonNull<bindings::ClearCore_Connector> {
        unsafe {
            let base_ptr = addr_of_mut!((*self.ptr.as_ptr())._base);
            NonNull::new_unchecked(base_ptr)
        }
    }
}
