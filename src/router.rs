use crate::{Mailbox, Message, MessagingError};
use agentropic_core::AgentId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Message router
#[derive(Debug, Clone)]
pub struct Router {
    mailboxes: Arc<RwLock<HashMap<AgentId, Mailbox>>>,
}

impl Router {
    /// Create new router
    pub fn new() -> Self {
        Self {
            mailboxes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an agent with a mailbox
    pub fn register(&self, agent_id: AgentId, mailbox: Mailbox) -> Result<(), MessagingError> {
        let mut mailboxes = self
            .mailboxes
            .write()
            .map_err(|_| MessagingError::LockError)?;
        mailboxes.insert(agent_id, mailbox);
        Ok(())
    }

    /// Unregister an agent
    pub fn unregister(&self, agent_id: &AgentId) -> Result<(), MessagingError> {
        let mut mailboxes = self
            .mailboxes
            .write()
            .map_err(|_| MessagingError::LockError)?;
        mailboxes.remove(agent_id);
        Ok(())
    }

    /// Send a message
    pub fn send(&self, message: Message) -> Result<(), MessagingError> {
        let mailboxes = self
            .mailboxes
            .read()
            .map_err(|_| MessagingError::LockError)?;

        let mailbox = mailboxes
            .get(&message.receiver())
            .ok_or(MessagingError::AgentNotFound)?;

        mailbox
            .send(message)
            .map_err(MessagingError::SendFailed) 
    }

    /// Create a new mailboxx
    pub fn create_mailbox(&self) -> Mailbox {
        Mailbox::new()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
