use serde::{Deserialize, Serialize};

/// Message performative (speech act)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Performative {
    Inform,
    Request,
    Query,
    Propose,
    Accept,
    Reject,
    Confirm,
    Disconfirm,
    Subscribe,
    CFP,
    Refuse,
}
