mod api;
use crate::api::socket::Socket;
use crate::api::thermometer::Thermometer;

fn main() {
    let therm = Thermometer::new();
    let mut socket = Socket::switch_on(therm);
    let consumption = socket.get_current_consumption();
    let temperature = socket.get_current_temperature();
    let description = socket.get_description();
    println!(
        "{0}, my consumption is {1}W and current temperature is {2} degrees",
        description, consumption, temperature
    );
    socket.switch_off();
}
