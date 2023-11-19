use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MessagesListItem {
    pub id: Uuid,
    pub case_id: Option<Uuid>,
    pub message_type: i16,
    pub message_group: String, // Replace with actual type
    pub created_at: NaiveDateTime,
    pub is_read: bool,
    pub message_text: String,
    pub case_type_id: Option<String>, // Replace with actual type
    pub case_number: String,
    pub number_of_unread_replys: i32,
    pub replys_count: i32,
    pub abbreviation: String,
    pub sender_name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MessageThreadItem {
    pub message_type: MessageTypes,
    pub created_at: NaiveDateTime,
    pub is_read: bool,
    pub message_text: String,
    pub case_number: String,
    pub sender_id: Option<Uuid>,
    pub sender_full_name: String,
    pub abbreviation: String,
    pub case_id: Option<Uuid>,
    pub lost_case_id: Option<Uuid>,
    pub lost_case_baggage_id: Option<Uuid>,
    pub is_from_current_user: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]

pub enum MessageTypes {
    LostBaggageCase = 1,
    MatchedBaggageFound,
    UnclaimedBaggage,
    ForwardedBaggage,
    MessageForUser,
    MessageForCaseRequestor,
    MessageForBaggageOwner,
    ReplyMessage,
    CaseDeletedMessage,
    SupportRequestMessage,
    BaggageDeliveryRequestMessage,
    BaggageDeliveryMessage,
    BaggageDeliveryWithoutCaseMessage,
    BaggageDeliveryRequestDeclinedMessage,
    BaggageDeliveryRequestConfirmedMessage,
    LostCaseConversation,
    UnclaimedBaggageConversation,
    DocumentsSoonToBeArchived,
    DamagedBaggageConversation,
    QuickDelivery,
    QuickDeliveryInitiated,
    QuickDeliveryIsDelivered,
    QuickDeliveryConversation,
    LostThingConversation,
    FoundThingConversation,
    BsmRushDelivery,
    ComplexMatching,
    AttachmentVersion,
    LinkToCommercialActPDF,
}
