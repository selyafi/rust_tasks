trait Device {
    fn new(name: String, value: String) -> Self;
    fn get_name(&self) -> &str;
    fn get_value(&self) -> &str;
}

struct Socket {
    name: String,
    value: String,
}

impl Device for Socket {
    fn new(name: String, value: String) -> Self {
        Socket {
            name,
            value,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_value(&self) -> &str {
        &self.value
    }
}

struct Thermometer {
    name: string,
    value: String,
}

impl Device for Thermometer {
    fn new(name: String, value: String) -> Self {
        Thermometer {
            name,
            value,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_value(&self) -> &str {
        &self.value
    }
}