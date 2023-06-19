use std::sync::Arc;
use crate::types::room::Room;

pub struct SmartHome {
    pub name: String,
    pub username: String,
    pub password: String,
    pub rooms: Vec<Arc<dyn Room>>
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

   pub fn add_room(&mut self, room: Arc<dyn Room>) {
        self.rooms.push(room);
    }

    pub fn get_rooms(&self) -> Vec<Arc<dyn Room>> {
      self.rooms.clone()
    }

    pub fn get_room(&self, name: &str) -> Option<Arc<dyn Room>> {
        for room in &self.rooms {
            if room.get_name() == name {
                return Some(room.clone());
            }
        }
        None
    }
}