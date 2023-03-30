/**!
 *  Connector Mode Traits
 */

use core::ops;


/// Trait for basic digital input
pub trait InputDigital {
    fn read_state(&self) -> LogicState;
}

pub trait InputDigitalFiltered {
    fn filter_length(&self) -> FilterLength;
    fn set_filter_length(&mut self, filter_length: FilterLength);
}

pub trait InputDigitalEdge {
    fn input_risen(&mut self) -> bool;
    fn input_fallen(&mut self) -> bool;
}

pub trait InputDigitalISR {
    fn register_isr<F>(
        &mut self,
        callback: extern "C" fn() -> (),
        trigger: InterruptTrigger,
        enable: bool,
    );
}

pub trait InputAnalog {
    fn set_filter_tc(tc: FilterTC);
    fn read_voltage(&self) -> f32;
}

/// Trait for basic digital output
pub trait OutputDigital {
    fn state(&self) -> LogicState;
    fn set_state(&mut self, state: LogicState);
}

pub trait OutputDigitalPulse {
    fn start_pulses(&mut self, on_time: u32, off_time: u32, count: u16);
    fn start_pulses_blocking(&mut self, on_time: u32, off_time: u32, count: u16);
    fn start_pulses_forever(&mut self, on_time: u32, off_time: u32);
    fn stop_pulses(&mut self);
    fn stop_pulses_immediately(&mut self);
    fn is_pulse_active(&self) -> bool;
}

pub trait OutputCurrent {
    fn set_current(microamps: u16);
}

pub trait OutputPWM {
    fn set_pwm(&mut self, duty: u8);
}

pub trait OutputHBridge {
    fn state(&self) -> i16;
    fn set_state(&mut self, state: i16);
}

pub trait OutputTone {
    // fn state(&self) -> ToneState;
    fn tone_amplitude(&mut self, amp: i16);
    fn tone_continuous(&mut self, freq: u16);
    fn tone_timed(&mut self, options: &TimedOpts);
    fn tone_periodic(&mut self, freq: u16, time_on: u32, time_off: u32);
    fn tone_stop(&mut self);
}


// Helpers

#[derive(Debug, Clone, Copy)]
pub enum LogicState {
    Low,
    High,
}

impl ops::Not for LogicState {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            LogicState::Low => LogicState::High,
            LogicState::High => LogicState::Low,
        }
    }
}

impl Into<bool> for LogicState {
    fn into(self) -> bool {
        match self {
            LogicState::Low => false,
            LogicState::High => true,
        }
    }
}

// Helper conversions to/from raw state
impl LogicState {
    pub(crate) fn from_raw(state: i16) -> Self {
        match state {
            0 => LogicState::Low,
            _ => LogicState::High,
        }
    }
    
    pub(crate) fn into_raw(self) -> i16 {
        match self {
            LogicState::Low => 0,
            LogicState::High => 1,
        }
    }
}

/// Digital transition filter length
#[derive(Debug, Clone, Copy)]
pub enum FilterLength {
    Milliseconds(u16),
    Samples(u16),
}

/// Units for the filter time constant. 
#[derive(Debug, Clone, Copy)]
pub enum FilterTC {
    Raw(u16),
    Milliseconds(u16),
    Samples(u16),
}

#[derive(Debug, Clone, Copy)]
pub enum InterruptTrigger {
    None, Low, High, Chance, Falling, Rising,
}

#[derive(Debug, Clone)]
pub struct TimedOpts {
    pub freq: u16, 
    pub duration: u32, 
    pub blocking: bool, 
    pub force_duration: bool,
}

impl TimedOpts {
    pub fn new(freq: u16, duration: u32) -> Self {
        TimedOpts { 
            freq, 
            duration,
            blocking: false,
            force_duration: false,
        }
    }
    
    pub fn blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking; self
    }
    
    pub fn force_duration(mut self, force: bool) -> Self {
        self.force_duration = force; self
    }
}
