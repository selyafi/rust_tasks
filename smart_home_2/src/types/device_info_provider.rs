use crate::{errors::smart_home_errors::SmartHomeError, types::report::Report};

pub trait DeviceInfoProvider {
    fn get_device_info(&self) -> Result<Report, SmartHomeError>;
}
