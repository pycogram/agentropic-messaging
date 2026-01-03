use crate::Message;
use crossbeam::channel::{unbounded, Receiver, Sender};

/// Agent mailbox for receiving messages
#[derive(Debug, Clone)]
pub struct Mailbox {
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}

impl Mailbox {
    /// Create a new mailbox
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { receiver, sender }
    }

    /// Get sender handle
    pub fn sender(&self) -> Sender<Message> {
        self.sender.clone()
    }

    /// Try to receive a message (non-blocking)
    pub fn try_receive(&self) -> Option<Message> {
        self.receiver.try_recv().ok()
    }

    /// Receive a message (blocking)
    pub fn receive(&self) -> Option<Message> {
        self.receiver.recv().ok()
    }

    /// Send a message to this mailbox
    pub fn send(&self, message: Message) -> Result<(), String> {
        self.sender
            .send(message)
            .map_err(|e| format!("Failed to send message: {}", e))
    }
}

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}
