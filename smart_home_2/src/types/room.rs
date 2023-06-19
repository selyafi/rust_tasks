use std::sync::Arc;
use crate::types::device::Device;
use crate::types::socket::Socket;

pub trait Room {
    fn new(name: String, socket: Socket) -> Self where Self: Sized;
    fn get_name(&self) -> &str;
    fn get_devices(&self) -> Vec<Arc<dyn Device>>;
    fn add_device(&mut self, device: Arc<dyn Device>);
    fn get_socket(&self) -> &Socket;
}