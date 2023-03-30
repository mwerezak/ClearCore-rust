/**!
 *  Connector Mode Traits
 */

use core::ops;


/// Trait for digital input modes
pub trait InputDigital {
    fn read_state(&self) -> LogicState;
}

/// Trait for digital output modes
pub trait OutputDigital {
    fn state(&self) -> LogicState;
    fn set_state(&mut self, state: LogicState);
}

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
