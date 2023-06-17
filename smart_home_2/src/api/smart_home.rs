struct SmartHome {
    pub name: String,
    pub username: String,
    pub password: String,
    pub devices: Vec<Device>,
    pub rooms: Vec<Room>
}

impl SmartHome {
    pub fn new(name: String, username: String, password: String) -> SmartHome {
        SmartHome {
            name,
            username,
            password,
            devices: Vec::new(),
            rooms: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn get_device(&self, name: &str) -> Option<&Device> {
        for device in &self.devices {
            if device.get_name() == name {
                return Some(device);
            }
        }
        None
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        for room in &self.rooms {
            if room.get_name() == name {
                return Some(room);
            }
        }
        None
    }
}