//! Prelude for convenient imports

pub use crate::message::{Message, MessageId};
pub use crate::builder::MessageBuilder;
pub use crate::performative::Performative;
pub use crate::router::Router;
pub use crate::mailbox::Mailbox;
pub use crate::error::MessagingError;

// Re-export from core
pub use agentropic_core::prelude::*;
