/*!
 *  
 */

pub mod modes;
pub mod led_driver;

use core::ptr::NonNull;
use crate::bindings;
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


// I'm not sure if the rest of this will ever be needed, but it's here...

/*
    Connector Mode enum
*/

type RawConnectorMode = bindings::ClearCore_Connector_ConnectorModes;

/// All possible operational modes for a connector. 
#[allow(non_camel_case_types)]
pub enum ConnectorMode {
    /// An invalid default mode.
    InvalidNone,
    /// Analog input mode.
    InputAnalog,
    /// Digital input mode.
    InputDigital,
    /// Analog current source output mode.
    OutputAnalog,
    /// Digital output mode.
    OutputDigital,
    /// H-Bridge mode, using differential PWM output.
    OutputHBridge,
    /// Periodic digital output mode, using pulse-width modulation (PWM).
    OutputPWM,
    /// Tone generation mode, using H-Bridge's differential 
    /// PWM output with tone generation features enabled.
    OutputTone,
    /// Audio generation mode, playing a wave file from a flash drive. 
    OutputWave,
    /// ClearPath™ motor controller mode, compatible with operational modes 
    /// that require user's direct control of the A and B input signals. 
    CPM_ADirect_BDirect,
    /// ClearPath™ motor controller mode, compatible with Step and Direction operational modes. 
    CPM_StepAndDir,
    /// ClearPath™ motor controller mode, compatible with operational modes where A is
    /// controlled by the user directly and B is controlled with a PWM signal
    /// (e.g., the Follow Digital Torque, Velocity, and/or Position commands). 
    CPM_ADirect_BPWM,
    /// ClearPath™ motor controller mode, compatible with Follow Digital Velocity: Bipolar PWM Command 
    /// with Variable Torque operational mode where both inputs A and B are controlled with PWM signals.
    CPM_APWM_BPWM,
    /// Serial port mode, using standard TTL levels compatible with USB Serial Bridges. 
    TTL,
    /// Serial port mode, using inverted TTL levels to allow direct RS232 connections for 
    /// ports tolerant of the lack of negative voltages.
    RS232,
    /// Serial port mode, using the port in SPI mode for connections to serial devices using this format.
    SPI,
    /// Serial port mode for CCIO-8 connections. 
    CCIO,
    /// Serial port mode for USB. 
    USB_CDC,
}

impl Into<RawConnectorMode> for ConnectorMode {
    fn into(self) -> RawConnectorMode {
        match self {
            Self::InvalidNone => bindings::ClearCore_Connector_ConnectorModes_INVALID_NONE,
            Self::InputAnalog => bindings::ClearCore_Connector_ConnectorModes_INPUT_ANALOG,
            Self::InputDigital => bindings::ClearCore_Connector_ConnectorModes_INPUT_DIGITAL,
            Self::OutputAnalog => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_ANALOG,
            Self::OutputDigital => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_DIGITAL,
            Self::OutputHBridge => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_H_BRIDGE,
            Self::OutputPWM => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_PWM,
            Self::OutputTone => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_TONE,
            Self::OutputWave => bindings::ClearCore_Connector_ConnectorModes_OUTPUT_WAVE,
            Self::CPM_ADirect_BDirect => bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_DIRECT,
            Self::CPM_StepAndDir => bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_STEP_AND_DIR,
            Self::CPM_ADirect_BPWM => bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_PWM,
            Self::CPM_APWM_BPWM => bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_PWM_B_PWM,
            Self::TTL => bindings::ClearCore_Connector_ConnectorModes_TTL,
            Self::RS232 => bindings::ClearCore_Connector_ConnectorModes_RS232,
            Self::SPI => bindings::ClearCore_Connector_ConnectorModes_SPI,
            Self::CCIO => bindings::ClearCore_Connector_ConnectorModes_CCIO,
            Self::USB_CDC => bindings::ClearCore_Connector_ConnectorModes_USB_CDC,
        }
    }
}

impl TryFrom<RawConnectorMode> for ConnectorMode {
    type Error = RawConnectorMode;
    fn try_from(value: RawConnectorMode) -> Result<Self, RawConnectorMode> {
        match value {
            bindings::ClearCore_Connector_ConnectorModes_INVALID_NONE => Ok(Self::InvalidNone),
            bindings::ClearCore_Connector_ConnectorModes_INPUT_ANALOG => Ok(Self::InputAnalog),
            bindings::ClearCore_Connector_ConnectorModes_INPUT_DIGITAL => Ok(Self::InputDigital),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_ANALOG => Ok(Self::OutputAnalog),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_DIGITAL => Ok(Self::OutputDigital),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_H_BRIDGE => Ok(Self::OutputHBridge),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_PWM => Ok(Self::OutputPWM),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_TONE => Ok(Self::OutputTone),
            bindings::ClearCore_Connector_ConnectorModes_OUTPUT_WAVE => Ok(Self::OutputWave),
            bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_DIRECT => Ok(Self::CPM_ADirect_BDirect),
            bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_STEP_AND_DIR => Ok(Self::CPM_StepAndDir),
            bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_PWM => Ok(Self::CPM_ADirect_BPWM),
            bindings::ClearCore_Connector_ConnectorModes_CPM_MODE_A_PWM_B_PWM => Ok(Self::CPM_APWM_BPWM),
            bindings::ClearCore_Connector_ConnectorModes_TTL => Ok(Self::TTL),
            bindings::ClearCore_Connector_ConnectorModes_RS232 => Ok(Self::RS232),
            bindings::ClearCore_Connector_ConnectorModes_SPI => Ok(Self::SPI),
            bindings::ClearCore_Connector_ConnectorModes_CCIO => Ok(Self::CCIO),
            bindings::ClearCore_Connector_ConnectorModes_USB_CDC => Ok(Self::USB_CDC),
            _ => Err(value),
        }
    }
}

/*
    Connector Type enum
*/
type RawConnectorType = bindings::ClearCore_Connector_ConnectorTypes;

pub enum ConnectorType {
    DigitalIn,
    DigitalInOut,
    ShiftReg,
    AnalogInDigitalIn,
    AnalogOutDigitalInOut,
    HBridge,
    CPM,
    Serial,
    SerialUSB,
    CCIODigitalInOut,
}

impl Into<RawConnectorType> for ConnectorType {
    fn into(self) -> RawConnectorType {
        match self {
            Self::DigitalIn => bindings::ClearCore_Connector_ConnectorTypes_DIGITAL_IN_TYPE,
            Self::DigitalInOut => bindings::ClearCore_Connector_ConnectorTypes_DIGITAL_IN_OUT_TYPE,
            Self::ShiftReg => bindings::ClearCore_Connector_ConnectorTypes_SHIFT_REG_TYPE,
            Self::AnalogInDigitalIn => bindings::ClearCore_Connector_ConnectorTypes_ANALOG_IN_DIGITAL_IN_TYPE,
            Self::AnalogOutDigitalInOut => bindings::ClearCore_Connector_ConnectorTypes_ANALOG_OUT_DIGITAL_IN_OUT_TYPE,
            Self::HBridge => bindings::ClearCore_Connector_ConnectorTypes_H_BRIDGE_TYPE,
            Self::CPM => bindings::ClearCore_Connector_ConnectorTypes_CPM_TYPE,
            Self::Serial => bindings::ClearCore_Connector_ConnectorTypes_SERIAL_TYPE,
            Self::SerialUSB => bindings::ClearCore_Connector_ConnectorTypes_SERIAL_USB_TYPE,
            Self::CCIODigitalInOut => bindings::ClearCore_Connector_ConnectorTypes_CCIO_DIGITAL_IN_OUT_TYPE,
        }
    }
}

impl TryFrom<RawConnectorType> for ConnectorType {
    type Error = RawConnectorType;
    fn try_from(value: RawConnectorType) -> Result<Self, RawConnectorType> {
        match value {
            bindings::ClearCore_Connector_ConnectorTypes_DIGITAL_IN_TYPE => Ok(Self::DigitalIn),
            bindings::ClearCore_Connector_ConnectorTypes_DIGITAL_IN_OUT_TYPE => Ok(Self::DigitalInOut),
            bindings::ClearCore_Connector_ConnectorTypes_SHIFT_REG_TYPE => Ok(Self::ShiftReg),
            bindings::ClearCore_Connector_ConnectorTypes_ANALOG_IN_DIGITAL_IN_TYPE => Ok(Self::AnalogInDigitalIn),
            bindings::ClearCore_Connector_ConnectorTypes_ANALOG_OUT_DIGITAL_IN_OUT_TYPE => Ok(Self::AnalogOutDigitalInOut),
            bindings::ClearCore_Connector_ConnectorTypes_H_BRIDGE_TYPE => Ok(Self::HBridge),
            bindings::ClearCore_Connector_ConnectorTypes_CPM_TYPE => Ok(Self::CPM),
            bindings::ClearCore_Connector_ConnectorTypes_SERIAL_TYPE => Ok(Self::Serial),
            bindings::ClearCore_Connector_ConnectorTypes_SERIAL_USB_TYPE => Ok(Self::SerialUSB),
            bindings::ClearCore_Connector_ConnectorTypes_CCIO_DIGITAL_IN_OUT_TYPE => Ok(Self::CCIODigitalInOut),
            _ => Err(value),
        }
    }
}
