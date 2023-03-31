
use core::ptr::{addr_of_mut, NonNull};

use crate::bindings;
use crate::connector::Connector;


pub struct DigitalInOut {
    ptr: NonNull<bindings::ClearCore_DigitalInOut>,
}

impl DigitalInOut {
    pub(crate) unsafe fn new(connector: &mut bindings::ClearCore_DigitalInOut) -> Self {
        DigitalInOut {
            ptr: NonNull::new_unchecked(addr_of_mut!(*connector))
        }
    }
    
    // pub fn as_input(&mut self) -> DigitalInOut_Input;
    // pub fn as_output(&mut self) -> DigitalInOut_Output;
    // pub fn as_pwm(&mut self) -> DigitalInOut_PWM;
}

impl Connector for DigitalInOut {
    fn struct_ptr(&self) -> NonNull<bindings::ClearCore_Connector> {
        unsafe {
            let base_ptr = addr_of_mut!((*self.ptr.as_ptr())._base._base);
            NonNull::new_unchecked(base_ptr)
        }
    }
}



// possible states
pub struct DigitalInOut_Input<'a> {
    connector: &'a mut DigitalInOut,
}

pub struct DigitalInOut_Output<'a> {
    connector: &'a mut DigitalInOut,
}


pub struct DigitalInOut_PWM<'a> {
    connector: &'a mut DigitalInOut,
}
