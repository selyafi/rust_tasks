trait Room {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn get_devices(&self) -> Vec<&Device>;
    fn add_device(&mut self, device: &Device);
}

struct LivingRoom {
    name: String,
    devices: Vec<Device>,
}

impl Room for LivingRoom {
    fn new(name: String) -> Self {
        LivingRoom {
            name,
            devices: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_devices(self) -> Vec<Device> {
        self.devices
    }

    fn add_device(mut self, device: &Device) {
        self.devices.push(device);
    }
}

struct Kitchen {
    name: String,
    devices: Vec<Device>,
}

impl Room for Kitchen {
    fn new(name: String) -> Self {
        Kitchen {
            name,
            devices: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_devices(self) -> Vec<Device> {
        self.devices
    }

    fn add_device(mut self, device: &Device) {
        self.devices.push(device);
    }
}

struct Bedroom {
    name: String,
    devices: Vec<Device>,
}

impl Room for Bedroom {
    fn new(name: String) -> Self {
        Bedroom {
            name,
            devices: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_devices(self) -> Vec<Device> {
        self.devices
    }

    fn add_device(mut self, device: &Device) {
        self.devices.push(device);
    }
}
