use crate::types::device::Device;

#[derive(Clone)]
pub struct Thermometer {
    name: String,
    value: String,
}

impl Device for Thermometer {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_value(&self) -> &str {
        &self.value
    }
}

impl Thermometer {
    pub fn new(name: String, value: String) -> Self {
        Thermometer { name, value }
    }
}