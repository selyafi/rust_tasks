pub struct Socket {
    room: String,
    name: String,
    value: String,
    is_on: bool,
}

impl Socket {
    pub fn new(room: String, name: String, value: String) -> Self {
        Socket { room, name, value, is_on: false }
    }

    pub fn get_name(&self) -> String {
        let name = format!("{}_{}", self.room, self.name);
        name
    }
    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
    pub fn _get_room(&self) -> String {
        self.room.to_string()
    }

    pub fn set_on(&mut self) {
        self.is_on = true;
    }
    pub fn set_off(&mut self) {
        self.is_on = false;
    }
}
