use socket_lib::{Command, SmartSocketClient};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let server_address = "127.0.0.1:7890";
    let mut client = SmartSocketClient::new(server_address).await.unwrap();

    loop {
        print!("Enter command (on-1/off-2/getvalue-3/ison-4/exit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = match input.trim() {
            "1" => Some(Command::On),
            "2" => Some(Command::Off),
            "3" => Some(Command::GetValue),
            "4" => Some(Command::IsOn),
            _ => None,
        };

        let response = match command {
            Some(command) => client.run_command(command).await.unwrap(),
            None => {
                println!("Exitting...");
                break;
            }
        };
        println!("Response: {}", response);
    }
}
