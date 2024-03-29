use crate::{
    errors::smart_home_errors::SmartHomeError,
    types::{device_info_provider::DeviceInfoProvider, report::Report, room::Room},
};

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

    pub fn get_rooms(&mut self) -> &mut Vec<Room> {
        &mut self.rooms
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.iter().find(|&room| room.get_name() == name)
    }

    pub fn create_report<T>(&self, info_provider: &T) -> Result<Report, SmartHomeError>
    where
        T: DeviceInfoProvider,
    {
        info_provider.get_device_info()
    }

    pub fn delete_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        let room_position = self
            .rooms
            .iter()
            .position(|room| room.get_name() == room_name);
        match room_position {
            Some(index) => {
                self.rooms.remove(index);
                Ok(())
            }
            None => Err(SmartHomeError::DeleteRoomFailure),
        }
    }
}
