/*!
 *  
 */

pub mod modes;
pub mod led_driver;
pub mod digital_inout;

use core::ptr::NonNull;
use crate::bindings;

// re-export connector I/O traits
pub use modes::*;


#[derive(Debug, Clone, Copy)]
pub struct ConnectorIndex(i32);


/// Trait for types that implement common connector methods
pub trait Connector {
    fn struct_ptr(&self) -> NonNull<bindings::ClearCore_Connector>;

    fn is_in_hw_fault(&self) -> bool {
        unsafe {
            bindings::vmethod_call!(ClearCore_Connector_IsInHwFault: self.struct_ptr())
        }
    }

    fn refresh(&mut self) {
        unsafe {
            bindings::vmethod_call!(ClearCore_Connector_Refresh: self.struct_ptr())
        }
    }

    fn reinitialize(&mut self) {
        unsafe {
            (*self.struct_ptr().as_ptr()).Reinitialize();
        }
    }

    fn connector_index(&self) -> ConnectorIndex { 
        unsafe {
            ConnectorIndex((*self.struct_ptr().as_ptr()).ConnectorIndex())
        }
    }

    fn reg_mask(&self) -> u32 { 
        unsafe {
            (*self.struct_ptr().as_ptr()).InputRegMask()
        }
    }
}

/// Helper trait for connectors that can change modes
pub(crate) trait ConnectorModes: Connector {
    fn connector_mode(&self) -> bindings::ConnectorMode {
        unsafe {
            bindings::vmethod_call!(ClearCore_Connector_Mode: self.struct_ptr())
                .try_into().unwrap()
        }
    }

    fn set_connector_mode(&mut self, mode: bindings::ConnectorMode) {
        unsafe {
            bindings::vmethod_call!(ClearCore_Connector_Mode1: self.struct_ptr(), mode.into());
        }
    }

}


// fn set_connector_mode()

    // pub ClearCore_Connector_Mode:
    //     unsafe extern "C" fn(this: *mut ClearCore_Connector) -> ClearCore_Connector_ConnectorModes,
    // pub ClearCore_Connector_Mode1: unsafe extern "C" fn(
    //     this: *mut ClearCore_Connector,
    //     newMode: ClearCore_Connector_ConnectorModes,
    // ) -> bool,