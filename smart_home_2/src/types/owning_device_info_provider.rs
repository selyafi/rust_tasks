use crate::types::device::Device;
use crate::types::device_info_provider::DeviceInfoProvider;
use crate::types::report::Report;

pub struct OwningDeviceInfoProvider<'a> {
    device: &'a dyn Device,
}

impl<'a> OwningDeviceInfoProvider<'a> {
    pub fn new(device: &'a dyn Device) -> Self {
        OwningDeviceInfoProvider { device }
    }
}

impl<'a> DeviceInfoProvider for OwningDeviceInfoProvider<'a> {
    fn get_device_info(&self) -> Result<Report, String> {
        let device = self.device;
        match (device.get_room(), device.get_value()) {
            (room, value ) if !room.is_empty() && !value.is_empty() => Ok(Report {
                room,
                socket: value,
                device: "".to_string(),
                value: "".to_string(),
            }),
            _ => Err("Room or socket is empty".to_string()),
        }
    }
}
