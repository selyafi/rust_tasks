use std::{net::{SocketAddr, UdpSocket}, time::{Instant, Duration}, thread};

fn main() {
    let addr_ip = "127.0.0.1:4321";
    let args = std::env::args();
    let mut args = args.skip(1);
    let address = args.next().unwrap_or_else(|| addr_ip.into());

    let socket_addr = address
        .parse::<SocketAddr>()
        .expect("valid socket address expected");
        let addr_bind = "127.0.0.1:4320";
        let socket = UdpSocket::bind(addr_bind).expect("failed to bind socket");
        let start_value = Instant::now();
        println!("Start");
        loop {
            print!("..................");
            let time = start_value.elapsed().as_secs_f32();
            let temperature = get_demo_temperature(time);  
            print!("Temperature: {}", temperature);      
            let buf = temperature.to_be_bytes();
            socket.send_to(&buf, socket_addr).expect("failed to send data");
            thread::sleep(Duration::from_millis(100));
        }
}

fn get_demo_temperature(time: f32) -> f32 {
    let amplitude = 10.0;
    let period = 10.0;
    let offset = 20.0;
    amplitude * (2.0 * std::f32::consts::PI * time / period).sin() + offset
}
