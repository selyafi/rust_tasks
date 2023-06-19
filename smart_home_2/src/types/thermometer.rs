use crate::types::device::Device;
pub struct Thermometer {
    name: String,
    value: String,
}

impl Device for Thermometer {
    fn new(name: String, value: String) -> Self {
        Thermometer { name, value }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_value(&self) -> &str {
        &self.value
    }
}
