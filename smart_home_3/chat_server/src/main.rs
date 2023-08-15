use std::{fs, thread};
use std::error::Error;
use stp::server::{StpConnection, StpServer};
mod devices;
mod handler;

use devices::Devices;
use handler::{Request, RequestHandler};

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server: StpServer = StpServer::bind(addr)?;
    let devices = Devices::default();

    for connection in server.incoming() {
        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        let devices = devices.clone();
        thread::spawn(move || {
            if handle_connection(connection, devices).is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
    Ok(())
}

fn handle_connection(mut connection: StpConnection, devices: Devices) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(devices);
    loop {
        let req_str = connection.recv_request()?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req))?;
    }
}
