use crate::{Message, MessagingError};
use agentropic_core::AgentId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

/// Message router
#[derive(Debug, Clone)]
pub struct Router {
    senders: Arc<RwLock<HashMap<AgentId, mpsc::UnboundedSender<Message>>>>,
}

impl Router {
    /// Create new router
    pub fn new() -> Self {
        Self {
            senders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an agent and return its mailbox receiver
    pub fn register(&self, agent_id: AgentId) -> Result<mpsc::UnboundedReceiver<Message>, MessagingError> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut senders = self
            .senders
            .write()
            .map_err(|_| MessagingError::LockError)?;
        senders.insert(agent_id, sender);
        Ok(receiver)
    }

    /// Unregister an agent
    pub fn unregister(&self, agent_id: &AgentId) -> Result<(), MessagingError> {
        let mut senders = self
            .senders
            .write()
            .map_err(|_| MessagingError::LockError)?;
        senders.remove(agent_id);
        Ok(())
    }

    /// Send a message to the receiver's mailbox
    pub fn send(&self, message: Message) -> Result<(), MessagingError> {
        let senders = self
            .senders
            .read()
            .map_err(|_| MessagingError::LockError)?;

        let sender = senders
            .get(&message.receiver())
            .ok_or(MessagingError::AgentNotFound)?;

        sender
            .send(message)
            .map_err(|e| MessagingError::SendFailed(e.to_string()))
    }

    /// Check if an agent is registered
    pub fn is_registered(&self, agent_id: &AgentId) -> bool {
        self.senders
            .read()
            .map(|s| s.contains_key(agent_id))
            .unwrap_or(false)
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}