use std::{
    io::{Read, Write},
    net::TcpListener,
};

use socket_lib::{Command, Response};
fn main() {
    let server_addr = "127.0.0.1:7890";
    let listener = TcpListener::bind(server_addr).unwrap();
    let mut smart_socket = SmartSocket::default();
    println!("Server listening on {}", server_addr);

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let peer_addr = stream
                    .peer_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| "unknown".into());
                println!("Peer '{peer_addr}' connected");

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
            Err(err) => {
                println!("Can't receive connection: {}", err);
            }
        }
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
                Response::Disabled
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
