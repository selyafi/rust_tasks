use shl::socket;

pub struct ChatClient {
    socket: socket::Socket,
}


impl ChatClient {
    pub fn new(room: String, name: String, value: String) -> Self {
        let socket = socket::Socket::new(room, name, value);
        ChatClient { socket }
    }

    pub fn get_name(&self) -> String {
        self.socket.get_name()
    }

    pub fn get_value(&self) -> String {
        self.socket.get_value()
    }

    pub fn set_socket_on(&mut self) -> () {
        self.socket.set_on();
    }

    pub fn set_socket_off(&mut self) -> () {
        self.socket.set_off();
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
