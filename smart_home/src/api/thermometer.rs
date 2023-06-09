#[derive(Debug)]
pub struct Thermometer {
    is_on: bool,
    temperature: f32,
    consumption: u32,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            is_on: true,      // initialize
            temperature: 0.0, // starting temperature
            consumption: 25,  // default value for consumption.
        }
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    pub fn get_consumption(&self) -> u32 {
        self.consumption
    }
    pub fn switch_off(&mut self) {
        self.is_on = false;
    }
}
