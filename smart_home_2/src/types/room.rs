use crate::types::{device::Device, socket::Socket};

pub trait Room   {
    fn get_name(&self) -> &str;
    fn get_devices(&self) -> Vec<Box<&dyn Device>>;
    fn get_device(&self, name: String) -> Option<&dyn Device> ;
    fn add_device(&mut self, device: Box<dyn Device>);
    fn get_socket(&self) -> &Socket;
}