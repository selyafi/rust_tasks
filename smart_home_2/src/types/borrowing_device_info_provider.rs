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
    fn get_device_info(&self) -> Result<Report, String> {
        match (
            self.device.get_room(),
            self.device.get_name(),
            self.socket.get_name(),
            self.socket.get_value(),
        ) {
            (room, device, socket, socket_value)
                if !room.is_empty()
                    && !device.is_empty()
                    && !socket.is_empty()
                    && !socket_value.is_empty() =>
            {
                Ok(Report {
                    room,
                    socket,
                    device,
                    value: socket_value,
                })
            }
            _ => Err("Room or socket is empty or there are no values".to_string()),
        }
    }
}
