use crate::api::thermometer::Thermometer;

#[derive(Debug)]
pub struct Socket {
    is_on: bool,
    description: String,
    device: Thermometer,
}

impl Socket {
    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn switch_on(device: Thermometer) -> Socket {
        Socket {
            is_on: true,
            description: String::from("Hi, I am socket ;-)"),
            device, // add device, ideally should add device to the list
        }
    }

    pub fn switch_off(&mut self) {
        self.device.switch_off();
        self.is_on = false;
    }

    pub fn get_current_consumption(&self) -> u32 {
        self.device.get_consumption()
    }

    pub fn get_current_temperature(&self) -> f32 {
        self.device.get_temperature()
    }
}
