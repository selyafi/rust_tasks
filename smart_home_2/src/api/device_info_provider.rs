use crate::types::{device::Device, report::Report, socket::Socket};

pub struct OwningDeviceInfoProvider<'a> {
    device: &'a dyn Device,
}

impl<'a> OwningDeviceInfoProvider<'a> {
    pub fn new(device: &'a dyn Device) -> Self {
        OwningDeviceInfoProvider { device }
    }
    pub fn get_device_info(&self) -> Report {
        Report {
            room: self.device.get_name(),
            socket: self.device.get_value(),
            device: "".to_string(),
            value: "".to_string(),
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a Socket,
    device: &'b dyn Device,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a Socket, device: &'b dyn Device) -> Self {
        BorrowingDeviceInfoProvider { socket, device }
    }
    pub fn get_device_info(&self) -> Report {
        Report {
            room: self.device.get_room(),
            socket: self.socket.get_name(),
            device: self.device.get_name(),
            value: self.socket.get_value(),
        }
    }
}
