
use core::ptr::{addr_of_mut, NonNull};

use crate::bindings::{self, ConnectorMode};
use crate::connector::{
    Connector,
    ConnectorModeHelper,
    InputDigital,
    OutputDigital,
    OutputPWM,
};
use crate::connector::modes::{
    ModeState,
    InputDigitalState,
    OutputDigitalState,
    OutputPWMState,
}


pub struct DigitalInOut<S: ModeState> {
    ptr: NonNull<bindings::ClearCore_DigitalInOut>,
}

impl<S: ModeState> DigitalInOut<S> {
    pub(crate) unsafe fn new(connector: &mut bindings::ClearCore_DigitalInOut) -> DigitalInOut::<Unconfigured> {
        DigitalInOut::<Unconfigured> {
            ptr: NonNull::new_unchecked(addr_of_mut!(*connector))
        }
    }
    
    // change states

    pub fn into_input(self) -> DigitalInOut<InputDigitalState> {
        self.set_connector_mode(ConnectorMode::InputDigital);
        DigitalInOut::<InputDigitalState>(self)
    }
    pub fn into_output(self) -> DigitalInOut<OutputDigitalState> {
        self.set_connector_mode(ConnectorMode::OutputDigital);
        DigitalInOut::<OutputDigitalState>(self)
    }
    pub fn into_pwm(self) -> DigitalInOut<OutputPWMState> {
        self.set_connector_mode(ConnectorMode::OutputPWM);
        DigitalInOut::<OutputPWMState>(self)
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

impl ConnectorModeHelper for DigitalInOut { }


// Implement I/O traits

// impl InputDigital for DigitalInOut<InputDigitalState> {}
// impl OutputDigital for DigitalInOut<OutputDigitalState> {}
// impl OutputPWM for DigitalInOut<OutputPWMState> {}
