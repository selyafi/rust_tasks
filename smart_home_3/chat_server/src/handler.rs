use crate::devices::Devices;
use std::str::Split;

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}


pub struct RequestHandler {
    devices: Devices,
}

impl RequestHandler {
    pub fn new(devices: Devices) -> Self {
        Self { devices }
    }

    pub fn handle(&mut self,  mut request: Request) -> String {
        let command = request.next();
        match command {
            "get_value" => self.get_value(request),
            "set_socket" => self.set_socket(request),
            "is_on" => self.is_on(request),
            _ => "error".to_string(),
        }
    }

    fn get_value(&self, mut request: Request) -> String {
        let name = request.next();
        self.devices.get_socket_value(name)
    }

    fn set_socket(&mut self, mut request: Request) -> String {
        let room = request.next();
        let name = request.next();
        let value = request.next();
        self.devices.create_socket(room.to_string(), name.to_string(), value.to_string());
        "ok".to_string()
    }

    fn is_on(&mut self, mut request: Request) -> String {
        let name = request.next();
        let is_on = request.next();
        match is_on {
            "on" => self.devices.set_socket_on(name),
            "off" => self.devices.set_socket_off(name),
            _ => (),
        }
        "ok".to_string()
    }
}