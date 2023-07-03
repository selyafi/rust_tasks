// use crate::types::socket::Socket;
// use crate::types::{device::Device, room_trait::RoomTrait};
//
// pub struct Kitchen {
//     name: String,
//     devices: Vec<Box<dyn Device>>,
//     socket: Socket,
// }
//
// impl RoomTrait for Kitchen {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
//
//     fn get_devices(&self) -> Vec<&dyn Device> {
//         self.devices.iter().map(|device| device.as_ref()).collect()
//     }
//
//     fn add_device(&mut self, device: Box<dyn Device>) {
//         self.devices.push(device);
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
//     fn get_socket(&self) -> &Socket {
//         &self.socket
//     }
// }
//
// impl Kitchen {
//     pub fn new(name: String, socket: Socket) -> Self {
//         Kitchen {
//             name,
//             devices: Vec::new(),
//             socket,
//         }
//     }
// }
