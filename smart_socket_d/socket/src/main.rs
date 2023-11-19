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

// do testing while socket is running
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command_on() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::On);
        assert_eq!(response, Response::Ok);
        assert_eq!(smart_socket.enabled, true);
    }

    #[test]
    fn test_process_command_off() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::Off);
        assert_eq!(response, Response::Disabled);
        assert_eq!(smart_socket.enabled, false);
    }

    #[test]
    fn test_process_command_is_on_enabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = true;
        let response = smart_socket.process_command(Command::IsOn);
        assert_eq!(response, Response::Enabled);
    }

    #[test]
    fn test_process_command_is_on_disabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = false;
        let response = smart_socket.process_command(Command::IsOn);
        assert_eq!(response, Response::Disabled);
    }

    #[test]
    fn test_process_command_get_value_enabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = true;
        let response = smart_socket.process_command(Command::GetValue);
        assert_eq!(response, Response::Value(SmartSocket::POWER_ON));
    }

    #[test]
    fn test_process_command_get_value_disabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = false;
        let response = smart_socket.process_command(Command::GetValue);
        assert_eq!(response, Response::Value(SmartSocket::POWER_OFF));
    }

    #[test]
    fn test_process_command_unknown() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::Unknown);
        assert_eq!(response, Response::Unknown);
    }
}
