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

impl std::fmt::Display for Performative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}