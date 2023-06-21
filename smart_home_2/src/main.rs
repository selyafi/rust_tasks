use crate::api::{device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider}, smart_home};
use crate::types::{room::Room, living_room::LivingRoom, socket::Socket, kitchen::Kitchen, bed_room::Bedroom, thermometer::Thermometer};

mod api;
mod types;

fn main() {
    let mut smart_home = smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());
    let socket = Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut living_room = LivingRoom::new("Living Room".to_string(), socket);
    living_room.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(living_room));

    let socket = Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut kitchen = Kitchen::new("Kitchen".to_string(), socket);
    kitchen.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(kitchen));

    let socket = Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut bed_room = Bedroom::new("Bed Room".to_string(), socket);
    bed_room.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(bed_room));

    let living_room_socket = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        living_room.get_socket()
    };

    let info_provider = {
        let living_room = smart_home.get_room("Kitchen".to_string()).unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string()).clone();
        OwningDeviceInfoProvider::new(living_room_device_option.unwrap())
    };

    let info_provider2 = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string()).clone();
        let living_room_device = living_room_device_option.unwrap();
        BorrowingDeviceInfoProvider::new(living_room_socket, living_room_device)
    };

    let report1 = info_provider.get_device_info();
    let report2 = info_provider2.get_device_info();
    println!("Smart Home: {}", report1.0);
    println!("Smart Home: {}", report2.0);
}
