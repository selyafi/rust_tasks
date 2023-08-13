pub trait Device {
    fn get_room(&self) -> String;
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
}
