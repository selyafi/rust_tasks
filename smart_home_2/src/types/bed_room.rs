use crate::types::{device::Device, socket::Socket};
use crate::types::room::Room;

pub struct Bedroom {
    name: String,
    devices: Vec<Box<dyn Device>>,
    socket: Socket,
}

impl Room for Bedroom {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_devices(&self) -> Vec<Box<&dyn Device>> {
        self.devices.iter().map(|device| Box::new(device.as_ref()) as Box<&dyn Device>).collect()
    }

    fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    fn get_device(&self, name: String) -> Option<&dyn Device> {
        for device in &self.devices {
            if device.get_name() == name {
                let device = device.as_ref().clone();
                return Some(device);
            }
        }
        None
    }

    fn get_socket(&self) -> &Socket { &self.socket }
}

impl Bedroom {
    pub fn new(name: String, socket: Socket) -> Self {
        Bedroom {
            name,
            devices: Vec::new(),
            socket,
        }
    }
}