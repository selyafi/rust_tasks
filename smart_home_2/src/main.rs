use crate::api::smart_home;
use crate::types::{
    borrowing_device_info_provider::BorrowingDeviceInfoProvider,
    owning_device_info_provider::OwningDeviceInfoProvider,
};
use crate::types::{room::Room, socket::Socket, thermometer::Thermometer, tv::TV};

mod api;
pub mod errors;
mod tests;
mod types;

fn main() {
    let mut smart_home =
        smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());

    let mut living_room = Room::new(
        "Living Room".to_string(),
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
        let living_room_result = match smart_home.get_room("Living Room") {
            Some(room) => Ok(room),
            None => Err("Living Room not found".to_string()),
        };
        living_room_result.unwrap().get_socket()
    };

    let info_provider = {
        let living_room = smart_home.get_room("Kitchen").unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string());
        OwningDeviceInfoProvider::new(living_room_device_option.unwrap())
    };

    let info_provider2 = {
        let living_room = smart_home.get_room("Living Room").unwrap();
        let living_room_device_option = living_room.get_device("Thermometer".to_string());
        let living_room_device = living_room_device_option.unwrap();
        BorrowingDeviceInfoProvider::new(living_room_socket, living_room_device)
    };

    smart_home
        .create_report(&info_provider)
        .map(|report| {
            println!(
                "Smart Home: {}_{}_{}_{}",
                report.room, report.socket, report.device, report.value
            );
        })
        .unwrap_or_else(|error| println!("Error: {}", error));

    smart_home
        .create_report(&info_provider2)
        .map(|report| {
            println!(
                "Smart Home: {}_{}_{}_{}",
                report.room, report.socket, report.device, report.value
            );
        })
        .unwrap_or_else(|error| println!("Error: {}", error));

    println!("------------------------------");

    for room in smart_home.get_rooms() {
        println!("Room: {}", room.get_name());
        let socket = room.get_socket();
        println!("Socket: {}", socket.get_name());
        let devices = room.get_devices();
        devices
            .map(|devices| {
                for device in devices {
                    println!("Device: {}", device.get_name());
                }
            })
            .unwrap_or_else(|error| println!("Error: {}", error));
        println!("------------------------------");
    }

    println!("-------Delete phase----------");
    for room in smart_home.get_rooms() {
        println!("Room: {}", room.get_name());
        room.remove_device("Thermometer");
        println!("removed Thermometer");
        let devices = room.get_devices();
        devices
            .map(|devices| {
                for device in devices {
                    println!("Device: {}", device.get_name());
                }
            })
            .unwrap_or_else(|error| println!("Error: {}", error));

        println!("------ Split ----------");
    }

    if let Some(error) = smart_home.delete_room("Bed Room").err() {
        println!("Error: {}", error)
    }

    smart_home.get_rooms().iter().for_each(|room| {
        println!("Room: {}", room.get_name());
    });
}
