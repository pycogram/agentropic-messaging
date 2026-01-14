//! Communication protocols, message routing, and ACL for multi-agent systems.

//#![warn(missing_docs)]
#![allow(missing_docs)]

pub mod builder;
pub mod error;
pub mod mailbox;
pub mod message;
pub mod performative;
pub mod prelude;
pub mod protocols;
pub mod router;

// Re-exports
pub use builder::MessageBuilder;
pub use error::MessagingError;
pub use mailbox::Mailbox;
pub use message::{Message, MessageId};
pub use performative::Performative;
pub use router::Router;
