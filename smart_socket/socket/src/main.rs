use std::io::{Read, Write};

use socket_lib::{Command, Response};
fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let server_addr = args.next().unwrap_or_else(|| "127.0.0.1:7890".to_owned());
    let listener = std::net::TcpListener::bind(&server_addr).unwrap();
    let mut smart_socket = SmartSocket::default();
    println!("Server listening on {}", server_addr);
    for connection in listener.incoming() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("Can't receive connection: {}", err);
                continue;
            }
        };

        let peer_addr = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{}' connected", peer_addr);

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 5] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer_addr} lost. Waiting for new connections...");
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    const POWER_ON: f32 = 220.5;
    const POWER_OFF: f32 = 0.0;

    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::On => {
                self.enabled = true;
                Response::Ok
            }
            Command::Off => {
                self.enabled = false;
                Response::Ok
            }
            Command::IsOn => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetValue => {
                let value = match self.enabled {
                    true => Self::POWER_ON,
                    false => Self::POWER_OFF,
                };
                Response::Value(value)
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
