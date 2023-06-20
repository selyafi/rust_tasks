use crate::types::device::Device;
use crate::types::report::Report;
use crate::types::socket::Socket;

pub struct OwningDeviceInfoProvider<'a> {
    device: &'a dyn Device,
}

impl<'a> OwningDeviceInfoProvider<'a> {
   pub fn new(device: &'a dyn Device) -> Self {
        OwningDeviceInfoProvider { device }
    }
    pub fn get_device_info(&self) -> Report {
        Report(String::from(self.device.get_name()), String::from(self.device.get_value()), "".to_string(), "".to_string())
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
    pub fn get_device_info(&self) -> Report{
            return Report(String::from(self.device.get_name()), String::from(self.device.get_value()), String::from(self.socket.get_name()), String::from(self.socket.get_value()))
    }
}

