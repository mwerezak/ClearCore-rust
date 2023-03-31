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
            let vtable = self.struct_ptr().as_ref().vtable_;
            let vmethod = (*vtable).ClearCore_Connector_IsInHwFault;
            vmethod(self.struct_ptr().as_ptr())
        }
    }

    fn refresh(&mut self) {
        unsafe {
            let vtable = self.struct_ptr().as_ref().vtable_;
            let vmethod = (*vtable).ClearCore_Connector_Refresh;
            vmethod(self.struct_ptr().as_ptr())
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
