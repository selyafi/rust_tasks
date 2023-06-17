struct Report {
    name: String,
    value: String,
}

trait DeviceInfoProvider {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Option<&Report>;
}

struct OwningDeviceInfoProvider {
    device: Device
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Option<&Report> {
        if self.device.get_name() == device_name {
            return Some(&Report{
                name: self.device.get_name(),
                value: self.device.get_value(),
            });
        }
        None
    }
}
