use crate::types::device::Device;

pub struct TV {
    room: String,
    name: String,
    value: String,
}

impl Device for TV {
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

impl TV {
    pub fn new(room: String, name: String, value: String) -> Self {
        TV { room, name, value }
    }
}
