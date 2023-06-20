use crate::api::device_info_provider::BorrowingDeviceInfoProvider;
use crate::types::room::Room;

mod types;
mod api;

fn main() {
    let mut smart_home = api::smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());
    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut living_room = types::living_room::LivingRoom::new("Living Room".to_string(), socket);
    living_room.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(living_room));

    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut kitchen = types::kitchen::Kitchen::new("Kitchen".to_string(), socket);
    kitchen.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(kitchen));

    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut bed_room = types::bed_room::Bedroom::new("Bed Room".to_string(), socket);
    bed_room.add_device(Box::new(thermometer));
    smart_home.add_room(Box::new(bed_room));

    let living_room_socket = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        living_room.get_socket()
    };

    let info_provider2 = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string()).clone();
        let living_room_device = living_room_device_option.unwrap();
        BorrowingDeviceInfoProvider::new(living_room_socket, living_room_device)
    };

    let report = info_provider2.get_device_info();
    println!("Smart Home: {}", report.0);
}
