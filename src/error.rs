use thiserror::Error;

/// Messaging errors
#[derive(Error, Debug)]
pub enum MessagingError {
    #[error("Agent not found")]
    AgentNotFound,

    #[error("Send failed: {0}")]
    SendFailed(String),

    #[error("Lock error")]
    LockError,

    /// Other error
    #[error("Messaging error: {0}")]
    Other(String),
}
