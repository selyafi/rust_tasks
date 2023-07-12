use crate::types::device::Device;
use crate::types::socket::Socket;

pub struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
    socket: Socket,
}

impl Room {
    pub fn new(name: String, socket: Socket, devices: Vec<Box<dyn Device>>) -> Self {
        Self {
            name,
            socket,
            devices,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_device(&self, name: String) -> Option<&dyn Device> {
        self.devices
            .iter()
            .find(|d| d.get_name() == name)
            .map(|d| d.as_ref())
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    pub fn get_devices(&self) -> Result<Vec<&dyn Device>, String> {
        let devices = &self.devices;
        match devices {
            devices if !devices.is_empty() => {
                Ok(devices.iter().map(|device| device.as_ref()).collect())
            }
            _ => Err(format!("No devices in room {}", self.name)),
        }
    }

    pub fn get_socket(&self) -> &Socket {
        &self.socket
    }
}
