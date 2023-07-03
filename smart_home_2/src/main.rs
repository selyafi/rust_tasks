use crate::api::{
    device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
    smart_home,
};
use crate::types::{room::Room, socket::Socket, thermometer::Thermometer, tv::TV};

mod api;
mod tests;
mod types;

fn main() {
    let mut smart_home =
        smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());

    let mut living_room = Room::new(
        "Living_Room".to_string(),
        Socket::new(
            "Living Room".to_string(),
            "Socket".to_string(),
            "On".to_string(),
        ),
        vec![
            Box::new(Thermometer::new(
                "Living Room".to_string(),
                "Thermometer".to_string(),
                "On".to_string(),
            )),
            Box::new(TV::new(
                "Living Room".to_string(),
                "Tv".to_string(),
                "On".to_string(),
            )),
        ],
    );
    living_room.add_device(Box::new(TV::new(
        "Living Room".to_string(),
        "TV2".to_string(),
        "On".to_string(),
    )));
    smart_home.add_room(living_room);

    let kitchen = Room::new(
        "Kitchen".to_string(),
        Socket::new(
            "Kitchen".to_string(),
            "Socket".to_string(),
            "On".to_string(),
        ),
        vec![Box::new(Thermometer::new(
            "Kitchen".to_string(),
            "Thermometer".to_string(),
            "On".to_string(),
        ))],
    );
    smart_home.add_room(kitchen);

    let bed_room = Room::new(
        "Bed Room".to_string(),
        Socket::new(
            "Bed Room".to_string(),
            "Socket".to_string(),
            "On".to_string(),
        ),
        vec![Box::new(Thermometer::new(
            "Bed Room".to_string(),
            "Thermometer".to_string(),
            "On".to_string(),
        ))],
    );

    smart_home.add_room(bed_room);

    let living_room_socket = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        living_room.get_socket()
    };

    let info_provider = {
        let living_room = smart_home.get_room("Kitchen".to_string()).unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string());
        OwningDeviceInfoProvider::new(living_room_device_option.unwrap())
    };

    let info_provider2 = {
        let living_room = smart_home.get_room("Living Room".to_string()).unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string());
        let living_room_device = living_room_device_option.unwrap();
        BorrowingDeviceInfoProvider::new(living_room_socket, living_room_device)
    };

    let report1 = info_provider.get_device_info();
    let report2 = info_provider2.get_device_info();
    println!(
        "Smart Home: {}_{}_{}_{}",
        report1.room, report1.socket, report1.device, report1.value
    );
    println!(
        "Smart Home: {}_{}_{}_{}",
        report2.room, report2.socket, report2.device, report2.value
    );

    println!("------------------------------");

    let rooms = smart_home.get_rooms();
    for room in rooms {
        println!("Room: {}", room.get_name());
        let socket = room.get_socket();
        println!("Socket: {}", socket.get_name());
        let devices = room.get_devices();
        for device in devices {
            println!("Device: {}", device.get_name());
        }
        println!("------------------------------");
    }
}
