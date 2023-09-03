use std::net::SocketAddr;

use thermometer::Thermometer;

fn main() {
    let socket_addr = "127.0.0.1:4321"
        .parse::<SocketAddr>().unwrap();
    let thermometer = Thermometer::new(socket_addr).unwrap();
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let temperature = thermometer.get();
        println!("Temperature: {}", temperature);
     }
}