use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SmartHomeError {
    NoRoom,
    NoSocket,
    NoDevice,
    NoValue,
}

impl Error for SmartHomeError {}

impl Display for SmartHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SmartHomeError::*;
        match self {
            NoRoom => write!(f, "no room"),
            NoSocket => write!(f, "no socket"),
            NoDevice => write!(f, "no device"),
            NoValue => write!(f, "no value"),
        }
    }
}
