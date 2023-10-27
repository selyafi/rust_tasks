use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use utilities::{Device, Room, SmartHome};

mod utilities;

struct AppState {
    pub smart_home: SmartHome,
}

// Define the API endpoints
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.smart_home)
}

#[post("/rooms")]
async fn add_room(room: web::Json<Room>, data: web::Data<AppState>) -> impl Responder {
    let mut smart_home = data.smart_home.clone();
    smart_home
        .rooms
        .insert(room.name.clone(), room.into_inner());
    HttpResponse::Ok().json(&smart_home)
}

#[delete("/rooms/{name}")]
async fn delete_room(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut smart_home = data.smart_home.clone();
    smart_home.rooms.remove(&name.into_inner());
    HttpResponse::Ok().json(&smart_home)
}

#[get("/rooms/{name}")]
async fn get_room(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match data.smart_home.rooms.get(&name.into_inner()) {
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
    let mut smart_home = data.smart_home.clone();
    let room = smart_home.rooms.get_mut(&name.into_inner());
    match room {
        Some(room) => {
            let device = device.into_inner();
            room.devices.insert(device.name.clone(), device);
            HttpResponse::Ok().json(&smart_home)
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
    let mut smart_home = data.smart_home.clone();
    let room = smart_home.rooms.get_mut(&room_name.into_inner());
    match room {
        Some(room) => {
            room.devices.remove(&device_name.into_inner());
            HttpResponse::Ok().json(&smart_home)
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
    let room = data.smart_home.rooms.get(&room_name);
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
    for (room_name, room) in &data.smart_home.rooms {
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
    HttpResponse::Ok().body(report)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let smart_home = SmartHome {
        name: "My Smart Home".to_string(),
        rooms: HashMap::new(),
    };
    let app_state = web::Data::new(AppState { smart_home });

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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
