// lib.rs

use std::boxed::Box;
use std::os::raw::c_void;
use std::sync::Mutex;

pub struct SmartSocket {
    state: Mutex<bool>,
    power_consumption: Mutex<f64>,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartSocket {
    pub fn new() -> Self {
        SmartSocket {
            state: Mutex::new(false),
            power_consumption: Mutex::new(0.0),
        }
    }

    pub fn turn_on(&self) {
        let mut state = self.state.lock().unwrap();
        *state = true;
    }

    pub fn turn_off(&self) {
        let mut state = self.state.lock().unwrap();
        *state = false;
    }

    pub fn get_state(&self) -> bool {
        let state = self.state.lock().unwrap();
        *state
    }

    pub fn set_power_consumption(&self, power: f64) {
        let mut power_consumption = self.power_consumption.lock().unwrap();
        *power_consumption = power;
    }

    pub fn get_power_consumption(&self) -> f64 {
        let power_consumption = self.power_consumption.lock().unwrap();
        *power_consumption
    }
}

#[no_mangle]
pub extern "C" fn create_smart_socket() -> *mut c_void {
    let smart_socket = Box::new(SmartSocket::new());
    Box::into_raw(smart_socket) as *mut c_void
}

#[no_mangle]
pub extern "C" fn delete_smart_socket(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr as *mut SmartSocket));
    }
}

/// # Safety
///
///
#[no_mangle]
pub unsafe extern "C" fn smart_socket_turn_on(smart_socket: *mut SmartSocket) {
    let smart_socket = unsafe {
        assert!(!smart_socket.is_null());
        &*smart_socket
    };
    let mut state = smart_socket.state.lock().unwrap();
    *state = true;
}

/// # Safety
///
///
#[no_mangle]
pub unsafe extern "C" fn smart_socket_turn_off(smart_socket: *mut SmartSocket) {
    let smart_socket = unsafe {
        assert!(!smart_socket.is_null());
        &*smart_socket
    };
    let mut state = smart_socket.state.lock().unwrap();
    *state = false;
}

/// # Safety
///
///
#[no_mangle]
pub unsafe extern "C" fn smart_socket_get_state(smart_socket: *mut SmartSocket) -> bool {
    let smart_socket = unsafe {
        assert!(!smart_socket.is_null());
        &*smart_socket
    };
    let state = smart_socket.state.lock().unwrap();
    *state
}

/// # Safety
///
///
#[no_mangle]
pub unsafe extern "C" fn smart_socket_set_power_consumption(
    smart_socket: *mut SmartSocket,
    power: f64,
) {
    let smart_socket = unsafe {
        assert!(!smart_socket.is_null());
        &*smart_socket
    };
    let mut power_consumption = smart_socket.power_consumption.lock().unwrap();
    *power_consumption = power;
}

/// # Safety
///
///
#[no_mangle]
pub unsafe extern "C" fn smart_socket_get_power_consumption(smart_socket: *mut SmartSocket) -> f64 {
    let smart_socket = unsafe {
        assert!(!smart_socket.is_null());
        &*smart_socket
    };
    let power_consumption = smart_socket.power_consumption.lock().unwrap();
    *power_consumption
}
