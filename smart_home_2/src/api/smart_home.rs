use crate::types::room::Room;

pub struct SmartHome {
    pub name: String,
    pub username: String,
    pub password: String,
    pub rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(name: String, username: String, password: String) -> SmartHome {
        SmartHome {
            name,
            username,
            password,
            rooms: Vec::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.iter().collect()
    }

    pub fn get_room(&self, name: String) -> Option<&Room> {
        self.rooms.iter().find(|&room| room.get_name() == name)
    }
}
