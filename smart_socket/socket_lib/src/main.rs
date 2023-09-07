use std::{
    error::Error,
    io::{self, Write},
};

use socket_lib::{Command, SmartSocketClient};

fn main() -> Result<(), Box<dyn Error>> {
    let server_address = "127.0.0.1:8080";
    let mut client = SmartSocketClient::new(server_address)?;

    loop {
        print!("Enter command (on/off/getvalue/ison): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = match input.trim().to_lowercase().as_str() {
            "on" => Command::On,
            "off" => Command::Off,
            "getvalue" => Command::GetValue,
            "ison" => Command::IsOn,
            _ => Command::Unknown,
        };

        let response = client.run_command(command)?;
        println!("Response: {}", response);
    }
}
