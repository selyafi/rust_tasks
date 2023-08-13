use crate::errors::smart_home_errors::SmartHomeError;
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

    pub fn get_devices(&self) -> Result<Vec<&dyn Device>, SmartHomeError> {
        let devices = &self.devices;
        match devices {
            devices if !devices.is_empty() => {
                Ok(devices.iter().map(|device| device.as_ref()).collect())
            }
            _ => Err(SmartHomeError::NoDevice),
        }
    }

    pub fn remove_device(&mut self, name: &str) -> Option<Box<dyn Device>> {
        let index = self.devices.iter().position(|d| d.get_name() == name);
        match index {
            Some(index) => Some(self.devices.remove(index)),
            None => None,
        }
    }

    pub fn get_socket(&self) -> &Socket {
        &self.socket
    }
}
