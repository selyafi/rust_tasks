use crate::types::device::Device;

pub struct Thermometer {
    room: String,
    name: String,
    value: String,
}

impl Device for Thermometer {
    fn get_room(&self) -> String {
        self.room.to_string()
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
    fn get_value(&self) -> String {
        self.value.to_string()
    }
}

impl Thermometer {
    pub fn new(room: String, name: String, value: String) -> Self {
        Thermometer { room, name, value }
    }
}
