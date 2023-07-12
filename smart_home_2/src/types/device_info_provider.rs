use crate::types::report::Report;

pub trait DeviceInfoProvider {
    fn get_device_info(&self) -> Result<Report, String>;
}
