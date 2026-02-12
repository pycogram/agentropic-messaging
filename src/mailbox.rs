use crate::Message;
use tokio::sync::mpsc;

/// Agent mailbox for receiving messages
#[derive(Debug)]
pub struct Mailbox {
    receiver: mpsc::UnboundedReceiver<Message>,
    sender: mpsc::UnboundedSender<Message>,
}

impl Mailbox {
    /// Create a new mailbox
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self { receiver, sender }
    }

    /// Get sender handle
    pub fn sender(&self) -> mpsc::UnboundedSender<Message> {
        self.sender.clone()
    }

    /// Try to receive a message (non-blocking)
    pub fn try_receive(&mut self) -> Option<Message> {
        self.receiver.try_recv().ok()
    }

    /// Receive a message (async, awaits until available)
    pub async fn receive(&mut self) -> Option<Message> {
        self.receiver.recv().await
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