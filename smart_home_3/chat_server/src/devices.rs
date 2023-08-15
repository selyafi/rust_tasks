use std::sync::Arc;
use dashmap::DashMap;
use shl::socket;

#[derive(Default, Clone)]
pub struct Devices {
    sockets: Arc<DashMap<String, socket::Socket>>,
}

impl Devices {
    pub fn get_socket_value(&self, name: &str) -> String {
        let socket = self.sockets.get(name).unwrap();
        socket.get_value()
    }

    pub fn create_socket(&mut self, room: String, name: String, value: String) -> () {
        let socket = socket::Socket::new(room, name, value);
        self.sockets.insert(socket.get_name(), socket);
    }

    pub fn set_socket_on(&mut self, name: &str) -> () {
        let mut socket = self.sockets.get_mut(name).unwrap();
        socket.set_on();
    }

    pub fn set_socket_off(&mut self, name: &str) -> () {
        let mut socket = self.sockets.get_mut(name).unwrap();
        socket.set_off();
    }
}