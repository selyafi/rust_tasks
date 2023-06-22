use crate::types::room::Room;

pub struct SmartHome {
    pub name: String,
    pub username: String,
    pub password: String,
    pub rooms: Vec<Box<dyn Room>>,
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

    pub fn add_room(&mut self, room: Box<dyn Room>) {
        self.rooms.push(room);
    }

    pub fn _get_rooms(&self) -> Vec<&dyn Room> {
        self.rooms.iter().map(|room| room.as_ref()).collect()
    }

    pub fn get_room(&self, name: String) -> Option<&dyn Room> {
        for room in &self.rooms {
            if room.get_name() == name {
                let room = room.as_ref();
                return Some(room);
            }
        }
        None
    }
}
