pub struct Socket {
    name: String,
    value: String,
}

impl Socket {
    pub fn new(name: String, value: String) -> Self {
        Socket { name, value }
    }

    pub  fn get_name(&self) -> &str {
        &self.name
    }

    pub  fn get_value(&self) -> &str {
        &self.value
    }
}