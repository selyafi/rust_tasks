use std::{
    error::Error,
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

#[derive(Debug)]
pub enum Command {
    Off,
    On,
    GetValue,
    IsOn,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Off,
            1 => Self::On,
            2 => Self::IsOn,
            3 => Self::GetValue,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Off => 0,
            Command::On => 1,
            Command::IsOn => 2,
            Command::GetValue => 3,
            Command::Unknown => 255,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Value(f32),
    Unknown,
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            [3, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Value(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => {}
            Response::Enabled => buffer[0] = 1,
            Response::Disabled => buffer[0] = 2,
            Response::Value(val) => {
                buffer[0] = 3;
                buffer[1..].copy_from_slice(&val.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}
impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Is Working"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Value(value) => write!(f, "Current Value: {}", value),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct SmartSocketClient {
    stream: TcpStream,
}

impl SmartSocketClient {
    pub fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address)?;
        Ok(Self { stream })
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command_off() -> Result<(), Box<dyn Error>> {
        let mut client = SmartSocketClient::new("127.0.0.1:7890")?;
        let response = client.run_command(Command::Off)?;
        assert_eq!(response, Response::Ok);
        Ok(())
    }

    #[test]
    fn test_run_command_on() -> Result<(), Box<dyn Error>> {
        let mut client = SmartSocketClient::new("127.0.0.1:7890")?;
        let response = client.run_command(Command::On)?;
        assert_eq!(response, Response::Enabled);
        Ok(())
    }

    #[test]
    fn test_run_command_get_value() -> Result<(), Box<dyn Error>> {
        let mut client = SmartSocketClient::new("127.0.0.1:7890")?;
        let response = client.run_command(Command::GetValue)?;
        assert_eq!(response, Response::Value(42.0));
        Ok(())
    }

    #[test]
    fn test_run_command_is_on() -> Result<(), Box<dyn Error>> {
        let mut client = SmartSocketClient::new("127.0.0.1:7890")?;
        let response = client.run_command(Command::IsOn)?;
        assert_eq!(response, Response::Disabled);
        Ok(())
    }

    #[test]
    fn test_run_command_unknown() -> Result<(), Box<dyn Error>> {
        let mut client = SmartSocketClient::new("127.0.0.1:7890")?;
        let response = client.run_command(Command::Unknown)?;
        assert_eq!(response, Response::Unknown);
        Ok(())
    }
}
