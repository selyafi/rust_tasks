use crate::types::device::Device;
use crate::types::room::Room;
use crate::types::socket::Socket;

pub struct Kitchen {
    name: String,
    devices: Vec<Box<dyn Device>>,
    socket: Socket,
}

impl Room for Kitchen {
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

impl Kitchen {
    pub fn new(name: String, socket: Socket) -> Self {
        Kitchen {
            name,
            devices: Vec::new(),
            socket,
        }
    }
}