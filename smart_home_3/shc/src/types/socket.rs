pub struct Socket {
    room: String,
    name: String,
    value: String,
}

impl Socket {
    pub fn new(room: String, name: String, value: String) -> Self {
        Socket { room, name, value }
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
}
