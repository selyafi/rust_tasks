pub trait Device {
    fn new(name: String, value: String) -> Self where Self: Sized;
    fn get_name(&self) -> &str;
    fn get_value(&self) -> &str;
}
