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
    fn is_in_hw_fault(&self) -> bool;
    fn refresh(&mut self);
    fn reinitialize(&mut self);
    fn connector_index(&self) -> ConnectorIndex;
    fn reg_mask(&self) -> u32;
}


/*
    Blanket impl for Connector
*/

/// Marker trait for types with a `ClearCore_Connector` base pointer.
/// Used to blanket impl the `Connector` trait
trait ConnectorBase {
    fn base_ptr(&self) -> NonNull<bindings::ClearCore_Connector>;
}

impl<T> Connector for T where T: ConnectorBase {
    fn is_in_hw_fault(&self) -> bool {
        _BasePtr(self.base_ptr()).is_in_hw_fault() 
    }

    fn refresh(&mut self) {
        _BasePtr(self.base_ptr()).refresh() 
    }

    fn reinitialize(&mut self) {
        _BasePtr(self.base_ptr()).reinitialize()
    }

    fn connector_index(&self) -> ConnectorIndex { 
        ConnectorIndex(_BasePtr(self.base_ptr()).connector_index())
    }

    fn reg_mask(&self) -> u32 { 
        _BasePtr(self.base_ptr()).reg_mask() 
    }
}

// Helper
#[derive(Debug, Clone, Copy)]
struct _BasePtr(NonNull<bindings::ClearCore_Connector>);

unsafe impl Send for _BasePtr { }

impl _BasePtr {
    pub fn is_in_hw_fault(&self) -> bool {
        unsafe {
            let vmethod = (*self.0.as_ref().vtable_).ClearCore_Connector_IsInHwFault;
            vmethod(self.0.as_ptr())
        }
    }
    
    pub fn refresh(&mut self) {
        unsafe {
            let vmethod = (*self.0.as_ref().vtable_).ClearCore_Connector_Refresh;
            vmethod(self.0.as_ptr());
        }
    }
    
    pub fn reinitialize(&mut self) {
        unsafe { (*self.0.as_ptr()).Reinitialize(); }
    }
    
    pub fn connector_index(&self) -> i32 {
        unsafe { (*self.0.as_ptr()).ConnectorIndex() }
    }
    
    pub fn reg_mask(&self) -> u32 {
        unsafe { (*self.0.as_ptr()).InputRegMask() }
    }
}
