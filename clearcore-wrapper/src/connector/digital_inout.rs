
use core::ptr::{addr_of_mut, NonNull};

use crate::bindings;
use crate::connector::ConnectorBase;


pub struct DigitalInOut {
    ptr: NonNull<bindings::ClearCore_DigitalInOut>,
}

impl DigitalInOut {
    pub(crate) unsafe fn new(connector: &mut bindings::ClearCore_DigitalInOut) -> Self {
        DigitalInOut {
            ptr: NonNull::new_unchecked(addr_of_mut!(*connector))
        }
    }
    
    // pub fn into_input(self) -> DigitalInOut_Input;
    // pub fn into_output(self) -> DigitalInOut_Output;
    // pub fn into_pwm(self) -> DigitalInOut_PWM;
}

impl ConnectorBase for DigitalInOut {
    fn base_ptr(&self) -> NonNull<bindings::ClearCore_Connector> {
        unsafe {
            let base_ptr = addr_of_mut!((*self.ptr.as_ptr())._base._base);
            NonNull::new_unchecked(base_ptr)
        }
    }
}



// possible states
pub struct DigitalInOut_Input {
    inout: DigitalInOut,
}

impl DigitalInOut_Input {
    pub fn switch_mode(self) -> DigitalInOut { self.inout }
}

pub struct DigitalInOut_Output {
    inout: DigitalInOut,
}

impl DigitalInOut_Output {
    pub fn switch_mode(self) -> DigitalInOut { self.inout }
}

pub struct DigitalInOut_PWM {
    inout: DigitalInOut,
}

impl DigitalInOut_PWM {
    pub fn switch_mode(self) -> DigitalInOut { self.inout }
}
