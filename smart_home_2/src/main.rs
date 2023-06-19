use std::sync::Arc;
use crate::types::device::Device;
use crate::types::room::Room;

mod types;
mod api;

fn main() {
    let mut smart_home = api::smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());
    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut living_room = types::living_room::LivingRoom::new("Living Room".to_string(), socket);
    living_room.add_device(Arc::new(thermometer));
    smart_home.add_room(Arc::new(living_room));

    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut kitchen = types::kitchen::Kitchen::new("Kitchen".to_string(), socket);
    kitchen.add_device(Arc::new(thermometer));
    smart_home.add_room(Arc::new(kitchen));

    let socket = types::socket::Socket::new("Socket".to_string(), "On".to_string());
    let thermometer = types::thermometer::Thermometer::new("Thermometer".to_string(), "On".to_string());

    let mut bed_room = types::bed_room::Bedroom::new("Bed Room".to_string(), socket);
    bed_room.add_device(Arc::new(thermometer));
    smart_home.add_room(Arc::new(bed_room));

    println!("Smart Home: {}", smart_home.username);
}
