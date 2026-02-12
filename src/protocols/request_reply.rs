use crate::{Message, MessagingError, Router};
use std::time::Duration;

/// Request-Reply protocol helper
pub struct RequestReply {
    router: Router,
    timeout: Duration,
}

impl RequestReply {
    /// Create a new RequestReply protocol handler
    pub fn new(router: Router, timeout: Duration) -> Self {
        Self { router, timeout }
    }

    /// Send a request and await a reply
    pub async fn send_request(&self, msg: Message) -> Result<Message, MessagingError> {
        // TODO: implement request-reply protocol
        self.router.send(msg)?;
        Err(MessagingError::Other("request-reply not yet implemented".into()))
    }
}