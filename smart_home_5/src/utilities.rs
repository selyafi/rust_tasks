use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the SmartHome struct
#[derive(Serialize, Deserialize, Clone)]
pub struct SmartHome {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

// Define the Room struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Device>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
}

impl Device {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_device_info(&self) -> String {
        "Device".to_string()
    }
}
