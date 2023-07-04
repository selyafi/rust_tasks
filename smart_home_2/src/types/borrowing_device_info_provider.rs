use crate::types::device::Device;
use crate::types::device_info_provider::DeviceInfoProvider;
use crate::types::report::Report;
use crate::types::socket::Socket;

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a Socket,
    device: &'b dyn Device,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a Socket, device: &'b dyn Device) -> Self {
        BorrowingDeviceInfoProvider { socket, device }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self) -> Report {
        Report {
            room: self.device.get_room(),
            socket: self.socket.get_name(),
            device: self.device.get_name(),
            value: self.socket.get_value(),
        }
    }
}
