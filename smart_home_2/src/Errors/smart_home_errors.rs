use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHomeError {
    #[error("no room")]
    NoRoom,
    #[error("no socket")]
    NoSocket,
    #[error("no device")]
    NoDevice,
    #[error("no value")]
    NoValue,
    #[error("failed to delete room")]
    DeleteRoomFailure,
}
