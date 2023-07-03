// use crate::types::socket::Socket;
// use crate::types::{device::Device, room_trait::RoomTrait};
//
// pub struct LivingRoom {
//     name: String,
//     devices: Vec<Box<dyn Device>>,
//     socket: Socket,
// }
//
// impl RoomTrait for LivingRoom {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
//
//     fn get_devices(&self) -> Vec<&dyn Device> {
//         self.devices.iter().map(|device| device.as_ref()).collect()
//     }
//
//     fn get_device(&self, name: String) -> Option<&dyn Device> {
//         for device in &self.devices {
//             if device.get_name() == name {
//                 let device = device.as_ref();
//                 return Some(device);
//             }
//         }
//         None
//     }
//
//     fn add_device(&mut self, device: Box<dyn Device>) {
//         self.devices.push(device);
//     }
//
//     fn get_socket(&self) -> &Socket {
//         &self.socket
//     }
// }
//
// impl LivingRoom {
//     pub fn new(name: String, socket: Socket) -> Self {
//         LivingRoom {
//             name,
//             devices: Vec::new(),
//             socket,
//         }
//     }
// }
