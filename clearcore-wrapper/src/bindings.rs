/*!
 *  Low-level internal wrapper API around the raw bindings created by bindgen.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


include!(concat!(env!("OUT_DIR"), "/libClearCore_bindings.rs"));


macro_rules! vmethod_call {
    ( $vmethod:ident : $this_non_null:expr , $( $arg:expr ),+ $(,)? ) => {
        {
            let vtable = $this_non_null.as_ref().vtable_;
            let vmethod = (*vtable).$vmethod;
            vmethod($this_non_null.as_ptr(), $( $arg ),+ )
        }
    };
    ( $vmethod:ident : $this_non_null:expr ) => {
        {
            let vtable = $this_non_null.as_ref().vtable_;
            let vmethod = (*vtable).$vmethod;
            vmethod($this_non_null.as_ptr())
        }
    };
}
pub(crate) use vmethod_call;

/*
    Connector Mode enum
*/

type RawConnectorMode = ClearCore_Connector_ConnectorModes;

/// All possible operational modes for a connector. 
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
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
            Self::InvalidNone => ClearCore_Connector_ConnectorModes_INVALID_NONE,
            Self::InputAnalog => ClearCore_Connector_ConnectorModes_INPUT_ANALOG,
            Self::InputDigital => ClearCore_Connector_ConnectorModes_INPUT_DIGITAL,
            Self::OutputAnalog => ClearCore_Connector_ConnectorModes_OUTPUT_ANALOG,
            Self::OutputDigital => ClearCore_Connector_ConnectorModes_OUTPUT_DIGITAL,
            Self::OutputHBridge => ClearCore_Connector_ConnectorModes_OUTPUT_H_BRIDGE,
            Self::OutputPWM => ClearCore_Connector_ConnectorModes_OUTPUT_PWM,
            Self::OutputTone => ClearCore_Connector_ConnectorModes_OUTPUT_TONE,
            Self::OutputWave => ClearCore_Connector_ConnectorModes_OUTPUT_WAVE,
            Self::CPM_ADirect_BDirect => ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_DIRECT,
            Self::CPM_StepAndDir => ClearCore_Connector_ConnectorModes_CPM_MODE_STEP_AND_DIR,
            Self::CPM_ADirect_BPWM => ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_PWM,
            Self::CPM_APWM_BPWM => ClearCore_Connector_ConnectorModes_CPM_MODE_A_PWM_B_PWM,
            Self::TTL => ClearCore_Connector_ConnectorModes_TTL,
            Self::RS232 => ClearCore_Connector_ConnectorModes_RS232,
            Self::SPI => ClearCore_Connector_ConnectorModes_SPI,
            Self::CCIO => ClearCore_Connector_ConnectorModes_CCIO,
            Self::USB_CDC => ClearCore_Connector_ConnectorModes_USB_CDC,
        }
    }
}

impl TryFrom<RawConnectorMode> for ConnectorMode {
    type Error = RawConnectorMode;
    fn try_from(value: RawConnectorMode) -> Result<Self, RawConnectorMode> {
        match value {
            ClearCore_Connector_ConnectorModes_INVALID_NONE => Ok(Self::InvalidNone),
            ClearCore_Connector_ConnectorModes_INPUT_ANALOG => Ok(Self::InputAnalog),
            ClearCore_Connector_ConnectorModes_INPUT_DIGITAL => Ok(Self::InputDigital),
            ClearCore_Connector_ConnectorModes_OUTPUT_ANALOG => Ok(Self::OutputAnalog),
            ClearCore_Connector_ConnectorModes_OUTPUT_DIGITAL => Ok(Self::OutputDigital),
            ClearCore_Connector_ConnectorModes_OUTPUT_H_BRIDGE => Ok(Self::OutputHBridge),
            ClearCore_Connector_ConnectorModes_OUTPUT_PWM => Ok(Self::OutputPWM),
            ClearCore_Connector_ConnectorModes_OUTPUT_TONE => Ok(Self::OutputTone),
            ClearCore_Connector_ConnectorModes_OUTPUT_WAVE => Ok(Self::OutputWave),
            ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_DIRECT => Ok(Self::CPM_ADirect_BDirect),
            ClearCore_Connector_ConnectorModes_CPM_MODE_STEP_AND_DIR => Ok(Self::CPM_StepAndDir),
            ClearCore_Connector_ConnectorModes_CPM_MODE_A_DIRECT_B_PWM => Ok(Self::CPM_ADirect_BPWM),
            ClearCore_Connector_ConnectorModes_CPM_MODE_A_PWM_B_PWM => Ok(Self::CPM_APWM_BPWM),
            ClearCore_Connector_ConnectorModes_TTL => Ok(Self::TTL),
            ClearCore_Connector_ConnectorModes_RS232 => Ok(Self::RS232),
            ClearCore_Connector_ConnectorModes_SPI => Ok(Self::SPI),
            ClearCore_Connector_ConnectorModes_CCIO => Ok(Self::CCIO),
            ClearCore_Connector_ConnectorModes_USB_CDC => Ok(Self::USB_CDC),
            _ => Err(value),
        }
    }
}

/*
    Connector Type enum
*/
type RawConnectorType = ClearCore_Connector_ConnectorTypes;

#[derive(Debug, Clone, Copy)]
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
            Self::DigitalIn => ClearCore_Connector_ConnectorTypes_DIGITAL_IN_TYPE,
            Self::DigitalInOut => ClearCore_Connector_ConnectorTypes_DIGITAL_IN_OUT_TYPE,
            Self::ShiftReg => ClearCore_Connector_ConnectorTypes_SHIFT_REG_TYPE,
            Self::AnalogInDigitalIn => ClearCore_Connector_ConnectorTypes_ANALOG_IN_DIGITAL_IN_TYPE,
            Self::AnalogOutDigitalInOut => ClearCore_Connector_ConnectorTypes_ANALOG_OUT_DIGITAL_IN_OUT_TYPE,
            Self::HBridge => ClearCore_Connector_ConnectorTypes_H_BRIDGE_TYPE,
            Self::CPM => ClearCore_Connector_ConnectorTypes_CPM_TYPE,
            Self::Serial => ClearCore_Connector_ConnectorTypes_SERIAL_TYPE,
            Self::SerialUSB => ClearCore_Connector_ConnectorTypes_SERIAL_USB_TYPE,
            Self::CCIODigitalInOut => ClearCore_Connector_ConnectorTypes_CCIO_DIGITAL_IN_OUT_TYPE,
        }
    }
}

impl TryFrom<RawConnectorType> for ConnectorType {
    type Error = RawConnectorType;
    fn try_from(value: RawConnectorType) -> Result<Self, RawConnectorType> {
        match value {
            ClearCore_Connector_ConnectorTypes_DIGITAL_IN_TYPE => Ok(Self::DigitalIn),
            ClearCore_Connector_ConnectorTypes_DIGITAL_IN_OUT_TYPE => Ok(Self::DigitalInOut),
            ClearCore_Connector_ConnectorTypes_SHIFT_REG_TYPE => Ok(Self::ShiftReg),
            ClearCore_Connector_ConnectorTypes_ANALOG_IN_DIGITAL_IN_TYPE => Ok(Self::AnalogInDigitalIn),
            ClearCore_Connector_ConnectorTypes_ANALOG_OUT_DIGITAL_IN_OUT_TYPE => Ok(Self::AnalogOutDigitalInOut),
            ClearCore_Connector_ConnectorTypes_H_BRIDGE_TYPE => Ok(Self::HBridge),
            ClearCore_Connector_ConnectorTypes_CPM_TYPE => Ok(Self::CPM),
            ClearCore_Connector_ConnectorTypes_SERIAL_TYPE => Ok(Self::Serial),
            ClearCore_Connector_ConnectorTypes_SERIAL_USB_TYPE => Ok(Self::SerialUSB),
            ClearCore_Connector_ConnectorTypes_CCIO_DIGITAL_IN_OUT_TYPE => Ok(Self::CCIODigitalInOut),
            _ => Err(value),
        }
    }
}
