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
    fn get_device_info(&self) -> Report {
        Report {
            room: self.device.get_room(),
            socket: self.device.get_value(),
            device: "".to_string(),
            value: "".to_string(),
        }
    }
}
