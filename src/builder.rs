use crate::{Message, MessageId, Performative};
use agentropic_core::AgentId;

/// Message builder
#[derive(Debug)]
pub struct MessageBuilder {
    sender: Option<AgentId>,
    receiver: Option<AgentId>,
    performative: Option<Performative>,
    content: Option<String>,
    conversation_id: Option<String>,
    in_reply_to: Option<MessageId>,
}

impl MessageBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
            performative: None,
            content: None,
            conversation_id: None,
            in_reply_to: None,
        }
    }

    /// Set sender
    pub fn sender(mut self, sender: AgentId) -> Self {
        self.sender = Some(sender);
        self
    }

    /// Set receiver
    pub fn receiver(mut self, receiver: AgentId) -> Self {
        self.receiver = Some(receiver);
        self
    }

    /// Set performative
    pub fn performative(mut self, performative: Performative) -> Self {
        self.performative = Some(performative);
        self
    }

    /// Set content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set conversation ID
    pub fn conversation_id(mut self, id: impl Into<String>) -> Self {
        self.conversation_id = Some(id.into());
        self
    }

    /// Set reply reference
    pub fn in_reply_to(mut self, id: MessageId) -> Self {
        self.in_reply_to = Some(id);
        self
    }

    /// Build the message
    pub fn build(self) -> Result<Message, crate::MessagingError> {
        let mut msg = Message::new(
            self.sender.ok_or(crate::MessagingError::Other("sender required".into()))?,
            self.receiver.ok_or(crate::MessagingError::Other("receiver required".into()))?,
            self.performative.ok_or(crate::MessagingError::Other("performative required".into()))?,
            self.content.ok_or(crate::MessagingError::Other("content required".into()))?,
        );
        if let Some(cid) = self.conversation_id {
            msg = msg.with_conversation_id(cid);
        }
        if let Some(reply) = self.in_reply_to {
            msg = msg.with_reply_to(reply);
        }
        Ok(msg)
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}


