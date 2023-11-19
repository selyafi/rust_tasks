use crate::models::{MessageThreadItem, MessageTypes, MessagesListItem};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use utoipa::OpenApi;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/all",
    tag = "Get all messages",
    responses(
        (status = 200, description= "Authenticated User", body = Response),
    )
)]
#[get("api/all")]
async fn get_all() -> impl Responder {
    let messages: Vec<MessagesListItem> = vec![
        // Add your messages here
    ];

    HttpResponse::Ok().json(messages)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_message_thread,
        get_all
    ),
    components(
        schemas(MessagesListItem, MessageThreadItem, MessageTypes)
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/{id}/thread",
    tag = "Get message thread by id",
    responses(
        (status = 200, description= "Authenticated User", body = Response),
    )
)]
#[get("api/{id}/thread")]
async fn get_message_thread(id: web::Path<String>) -> impl Responder {
    let timestamp = NaiveDateTime::from_timestamp_opt(0, 0);
    // Implement your logic to get message thread by id
    // For now, it returns a dummy data
    HttpResponse::Ok().json(vec![MessageThreadItem {
        message_type: MessageTypes::LostBaggageCase,
        created_at: timestamp.unwrap(),
        is_read: false,
        message_text: String::from("Test message"),
        case_number: String::from("Test case number"),
        sender_id: Some(Uuid::new_v4()),
        sender_full_name: String::from("Test sender"),
        abbreviation: String::from("Test abbreviation"),
        case_id: Some(Uuid::new_v4()),
        lost_case_id: Some(Uuid::new_v4()),
        lost_case_baggage_id: Some(Uuid::new_v4()),
        is_from_current_user: false,
    }])
}
