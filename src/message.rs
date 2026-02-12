use crate::Performative;
use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// Unique message identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Create a new message ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Agent message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    id: MessageId,
    sender: AgentId,
    receiver: AgentId,
    performative: Performative,
    content: String,
    conversation_id: Option<String>,
    in_reply_to: Option<MessageId>,
    timestamp: SystemTime,
}

impl Message {
    /// Create a new message
    pub fn new(
        sender: AgentId,
        receiver: AgentId,
        performative: Performative,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: MessageId::new(),
            sender,
            receiver,
            performative,
            content: content.into(),
            conversation_id: None,
            in_reply_to: None,
            timestamp: SystemTime::now(),
        }
    }

    /// Get message ID
    pub fn id(&self) -> MessageId {
        self.id
    }

    /// Get sender
    pub fn sender(&self) -> AgentId {
        self.sender
    }

    /// Get receiver
    pub fn receiver(&self) -> AgentId {
        self.receiver
    }

    /// Get performative
    pub fn performative(&self) -> Performative {
        self.performative
    }

    /// Get content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get conversation ID
    pub fn conversation_id(&self) -> Option<&str> {
        self.conversation_id.as_deref()
    }

    /// Get reply reference
    pub fn in_reply_to(&self) -> Option<MessageId> {
        self.in_reply_to
    }

    /// Get timestamp
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn with_conversation_id(mut self, id: String) -> Self {
        self.conversation_id = Some(id);
        self
    }

    pub fn with_reply_to(mut self, id: MessageId) -> Self {
        self.in_reply_to = Some(id);
        self
    }
}
