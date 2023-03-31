
use core::ptr::{addr_of_mut, NonNull};

use crate::bindings::{self, ConnectorMode};
use crate::connector::{Connector, ConnectorModes};


pub struct DigitalInOut {
    ptr: NonNull<bindings::ClearCore_DigitalInOut>,
}

impl DigitalInOut {
    pub(crate) unsafe fn new(connector: &mut bindings::ClearCore_DigitalInOut) -> Self {
        DigitalInOut {
            ptr: NonNull::new_unchecked(addr_of_mut!(*connector))
        }
    }
    
    pub fn as_input(&mut self) -> DigitalInOut_Input {
        self.set_connector_mode(ConnectorMode::InputDigital);
        DigitalInOut_Input(self)
    }
    pub fn as_output(&mut self) -> DigitalInOut_Output {
        self.set_connector_mode(ConnectorMode::OutputDigital);
        DigitalInOut_Output(self)
    }
    pub fn as_pwm(&mut self) -> DigitalInOut_PWM {
        self.set_connector_mode(ConnectorMode::OutputPWM);
        DigitalInOut_PWM(self)
    }
}

impl Connector for DigitalInOut {
    fn struct_ptr(&self) -> NonNull<bindings::ClearCore_Connector> {
        unsafe {
            let base_ptr = addr_of_mut!((*self.ptr.as_ptr())._base._base);
            NonNull::new_unchecked(base_ptr)
        }
    }
}

impl ConnectorModes for DigitalInOut { }


/*
    Connector Modes
*/

pub struct DigitalInOut_Input<'a>(&'a mut DigitalInOut);

pub struct DigitalInOut_Output<'a>(&'a mut DigitalInOut);

pub struct DigitalInOut_PWM<'a>(&'a mut DigitalInOut);
