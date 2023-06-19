use std::sync::Arc;
use crate::types::device::Device;
use crate::types::room::Room;
use crate::types::socket::Socket;

pub(crate) struct Kitchen {
    name: String,
    devices: Vec<Arc<dyn Device>>,
    socket: Socket,
}

impl Room for Kitchen {
    fn new(name: String,  socket: Socket) -> Self {
        Kitchen {
            name,
            devices: Vec::new(),
            socket,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_devices(&self) -> Vec<Arc<dyn Device>> {
        self.devices.clone()
    }

    fn add_device(&mut self, device: Arc<dyn Device>) {
        self.devices.push(device);
    }

    fn get_socket(&self) -> &Socket { &self.socket }
}