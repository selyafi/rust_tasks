use socket_lib::{Command, SmartSocketClient};

use eframe::*;

struct MyApp {
    client: SmartSocketClient,
    is_on: bool,
    response: String,
}

impl MyApp {
    fn new(server_address: &str) -> Self {
        let client = SmartSocketClient::new(server_address).unwrap();
        Self {
            client,
            is_on: false,
            response: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Command:");

                if ui.button("On").clicked() {
                    if let Ok(response) = self.client.run_command(Command::On) {
                        self.is_on = true;
                        self.response = format!("Response: {}", response);
                    }
                }

                if ui.button("Off").clicked() {
                    if let Ok(response) = self.client.run_command(Command::Off) {
                        self.is_on = false;
                        self.response = format!("Response: {}", response);
                    }
                }

                if ui.button("Get Value").clicked() {
                    if let Ok(response) = self.client.run_command(Command::GetValue) {
                        self.response = format!("Response: {}", response);
                    }
                }

                if ui.button("Is On").clicked() {
                    self.response = format!("Is On: {}", self.is_on);
                }

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.label(&self.response);
        });
    }
}

fn main() {
    let server_address = "127.0.0.1:7890";
    let app = MyApp::new(server_address);
    let boxed_app = Box::new(move |_ctx: &CreationContext| Box::new(app) as Box<dyn App>);
    if let Err(e) = eframe::run_native(
        "SmartSocket Client",
        eframe::NativeOptions::default(),
        boxed_app,
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
