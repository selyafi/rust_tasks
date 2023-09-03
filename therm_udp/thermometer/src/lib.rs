use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

trait DeviceValue {
    type Value;
    fn get(&self) -> Self::Value;
    fn set(&self, value: Self::Value);
}

#[derive(Default)]
struct Temperature {
    value: Mutex<f32>,
}

impl DeviceValue for Temperature {
    type Value = f32;
 fn get(&self) -> Self::Value {
        *self.value.lock().unwrap()
    }
    fn set(&self, value: Self::Value) {
        *self.value.lock().unwrap() = value;
    }
}

pub struct Thermometer {
    value: Arc<Temperature>,
    is_initialized: Arc<AtomicBool>,
}

impl Thermometer {
    pub fn new<T: ToSocketAddrs>(address: T) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        let is_initialized = Arc::new(AtomicBool::new(true));
        let temperature = Arc::new(Temperature::default());
        let temperature_clone = temperature.clone();
        let is_initialized_clone = is_initialized.clone();
        thread::spawn(move || loop {
            let initialized = is_initialized_clone.load(Ordering::SeqCst);
            if initialized {
                
            let mut buf = [0; 4];
            if let Err(err) = socket.recv_from(&mut buf) {
                println!("error recieving value: {err}");
            }

            let val = f32::from_be_bytes(buf);
            temperature_clone.set(val);
            } else {
               return;
            }

        });

        Ok(Self {
            value: temperature,
            is_initialized,
        })
    }

    pub fn get(&self) -> f32 {
        self.value.get()
    }
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        self.is_initialized.store(true, Ordering::SeqCst);
    }
}
