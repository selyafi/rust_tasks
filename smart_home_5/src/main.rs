use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use utilities::{Device, Room, SmartHome};

mod utilities;

struct AppState {
    pub smart_home: Arc<Mutex<SmartHome>>,
}

// Define the API endpoints
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let res = &data.smart_home.lock().unwrap().to_owned();
    HttpResponse::Ok().json(res)
}

#[post("/rooms")]
async fn add_room(room: web::Json<Room>, data: web::Data<AppState>) -> impl Responder {
    let mut smart_home = data.smart_home.lock().unwrap();
    smart_home
        .rooms
        .insert(room.name.clone(), room.into_inner());
    HttpResponse::Ok().json(&smart_home.to_owned())
}

#[delete("/rooms/{name}")]
async fn delete_room(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut smart_home = data.smart_home.lock().unwrap();
    smart_home.rooms.remove(&name.into_inner());
    HttpResponse::Ok().json(&smart_home.to_owned())
}

#[get("/rooms/{name}")]
async fn get_room(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let smart_home = data.smart_home.lock().unwrap();
    match smart_home.rooms.get(&name.into_inner()) {
        Some(room) => HttpResponse::Ok().json(room),
        None => HttpResponse::NotFound().body("Room not found"),
    }
}

#[post("/rooms/{name}/devices")]
async fn add_device(
    name: web::Path<String>,
    device: web::Json<Device>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut smart_home = data.smart_home.lock().unwrap();
    let room = smart_home.rooms.get_mut(&name.into_inner());
    match room {
        Some(room) => {
            let device = device.into_inner();
            room.devices.insert(device.name.clone(), device);
            HttpResponse::Ok().json(&smart_home.to_owned())
        }
        None => HttpResponse::NotFound().body("Room not found"),
    }
}

#[delete("/rooms/{room_name}/devices/{device_name}")]
async fn delete_device(
    room_name: web::Path<String>,
    device_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut smart_home = data.smart_home.lock().unwrap();
    let room = smart_home.rooms.get_mut(&room_name.into_inner());
    match room {
        Some(room) => {
            room.devices.remove(&device_name.into_inner());
            HttpResponse::Ok().json(&smart_home.to_owned())
        }
        None => HttpResponse::NotFound().body("Room not found"),
    }
}

#[get("/rooms/{room_name}/devices/{device_name}")]
async fn get_device_name(
    room_name: web::Path<String>,
    device_name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let device_name = device_name.into_inner();
    let room_name = room_name.into_inner();
    let smart_home = data.smart_home.lock().unwrap();
    let room = smart_home.rooms.get(&room_name);
    match room {
        Some(room) => match room.devices.get(&device_name) {
            Some(device) => {
                let name = device.get_name();
                HttpResponse::Ok().body(name)
            }
            None => HttpResponse::NotFound().body("Device not found"),
        },
        None => HttpResponse::NotFound().body("Room not found"),
    }
}

#[get("/report")]
async fn report(data: web::Data<AppState>) -> impl Responder {
    let mut report = String::new();
    let smart_home = data.smart_home.lock().unwrap().to_owned();
    for (room_name, room) in smart_home.rooms {
        report.push_str(&format!("Room: {}\n", room_name));
        for (device_name, device) in &room.devices {
            report.push_str(&format!(
                "Device: {}\n{}\n{}\n",
                device_name,
                device.get_device_info(),
                device.get_name()
            ));
        }
    }
    HttpResponse::Ok().json(report)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut smart_home = SmartHome {
        name: "My Smart Home".to_string(),
        rooms: HashMap::new(),
    };
    let mut room = Room {
        name: "room1".to_string(),
        devices: HashMap::new(),
    };
    room.devices.insert(
        "device1".to_string(),
        Device {
            name: "device1".to_string(),
        },
    );
    smart_home.rooms.insert("room1".to_string(), room);
    let app_state = web::Data::new(AppState {
        smart_home: Arc::new(Mutex::new(smart_home)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(add_room)
            .service(delete_room)
            .service(get_room)
            .service(add_device)
            .service(delete_device)
            .service(get_device_name)
            .service(report)
    })
    .bind("127.0.0.1:2333")?
    .run()
    .await
}
