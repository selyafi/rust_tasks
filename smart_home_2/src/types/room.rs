use crate::types::device::Device;
use crate::types::socket::Socket;

pub trait Room   {
    fn get_name(&self) -> &str;
    fn get_devices(&self) -> Vec<Box<&dyn Device>>;
    fn add_device(&mut self, device: Box<dyn Device>);
    fn get_socket(&self) -> &Socket;
}