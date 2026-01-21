# agentropic-messaging

[![Crates.io](https://img.shields.io/crates/v/agentropic-messaging.svg)](https://crates.io/crates/agentropic-messaging)
[![Documentation](https://docs.rs/agentropic-messaging/badge.svg)](https://docs.rs/agentropic-messaging)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Communication protocols, message routing, and Agent Communication Language (ACL) for multi-agent systems.**

`agentropic-messaging` provides the infrastructure for agents to communicate, coordinate, and collaborate. It implements message passing semantics, routing mechanisms, and standardized communication protocols based on FIPA ACL standards.

---

## Purpose

This crate provides:

- **Message Protocols**: Structured message formats with unique IDs
- **Agent Communication Language (ACL)**: FIPA-compliant speech acts (performatives)
- **Message Routing**: Intelligent delivery of messages between agents
- **Mailbox System**: Per-agent message queues
- **Communication Patterns**: Request-reply and other interaction protocols

---

## Implementation Status

 **Fully Implemented (v0.1.0)**:
- **Message** - Core message structure with IDs and performatives
- **Performative** - 12 FIPA speech acts (Inform, Request, Query, etc.)
- **Router** - Message routing with registration and delivery
- **Mailbox** - Agent message queues with async receive
- **Protocols** - Request-reply pattern implementation

All components have:
-  Working implementations
-  Comprehensive tests
-  Runnable examples
-  Full documentation

**Future (v0.2.0+)**:
- Builder pattern API for ergonomic message construction
- Topic-based pub/sub routing
- Advanced protocols (ContractNet, Auction)
- Content-based routing and filtering

---

## Core Concepts

### Agent Messages

Messages are the fundamental unit of agent communication:
```rust
use agentropic_messaging::prelude::*;

// Create a message
let message = Message::new(
    sender_id,
    receiver_id,
    Performative::Request,
    "Get market data"
);

Each message has:
- Unique MessageId (UUID-based)
- Sender AgentId
- Receiver AgentId  
- Performative (speech act type)
- Content (string payload)
```

### Performatives (Speech Acts)

Based on FIPA ACL and speech act theory:
```rust
use agentropic_messaging::Performative;

// Available performatives:
Performative::Inform        // Share information
Performative::Request       // Ask for action
Performative::Query         // Ask a question
Performative::Propose       // Suggest action
Performative::Accept        // Agree to proposal
Performative::Reject        // Decline proposal
Performative::Confirm       // Verify information
Performative::Disconfirm    // Contradict information
Performative::Subscribe     // Request updates
Performative::CFP           // Call for proposals
Performative::Refuse        // Decline to perform
Performative::Agree         // Commit to action
```

### Message Routing

The router delivers messages between agents:
```rust
use agentropic_messaging::prelude::*;

// Create router
let mut router = Router::new();

// Register agents
router.register(agent_a);
router.register(agent_b);

// Route a message
router.route(message).await?;

// Check if routed
if router.has_routed(&message_id) {
    println!("Message delivered");
}
```

### Mailbox System

Each agent has a mailbox for receiving messages:
```rust
use agentropic_messaging::prelude::*;

// Create mailbox
let mut mailbox = Mailbox::new();

// Send to mailbox
mailbox.send(message).await?;

// Receive from mailbox
if let Some(msg) = mailbox.receive().await {
    println!("Received: {}", msg.content());
}

// Check mailbox size
println!("Messages: {}", mailbox.size());
```

### Request-Reply Protocol

Synchronous request-response pattern:
```rust
use agentropic_messaging::protocols::prelude::*;

// Create request-reply protocol
let protocol = RequestReply::new(requester_id, responder_id);

// Send request
let request = Message::new(
    requester_id,
    responder_id,
    Performative::Request,
    "Get status"
);

protocol.send_request(request).await?;

// Receive reply
if let Some(reply) = protocol.receive_reply().await {
    println!("Reply: {}", reply.content());
}
```

---

## What's Included

### Core Types

- `Message` - Message structure with ID, sender, receiver, performative, content
- `MessageId` - Unique message identifier (UUID-based)
- `Performative` - 12 FIPA-compliant speech act types
- `Router` - Message routing engine with registration
- `Mailbox` - Agent message queue with async operations

### Routing

- `Router` - Central message routing
  - `register()` - Register agents
  - `route()` - Deliver messages
  - `has_routed()` - Check delivery status
- `Mailbox` - Per-agent message queue
  - `send()` - Add message
  - `receive()` - Get next message
  - `is_empty()` - Check if empty

### Protocols

- `RequestReply` - Request-response pattern
  - `send_request()` - Send request message
  - `receive_reply()` - Wait for reply
  - Support for timeouts and error handling

### Performatives (FIPA ACL)

Based on Foundation for Intelligent Physical Agents standards:
- **Assertives**: Inform, Confirm, Disconfirm
- **Directives**: Request, Query, Subscribe
- **Commissives**: Propose, Accept, Agree
- **Declaratives**: CFP (Call for Proposals)
- **Expressives**: Refuse, Reject

---

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agentropic-messaging = "0.1.0"
agentropic-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Message Passing
```rust
use agentropic_messaging::prelude::*;

#[tokio::main]
async fn main() -> Result<(), MessagingError> {
    // Create agents
    let agent_a = AgentId::new();
    let agent_b = AgentId::new();
    
    // Create mailboxes
    let mut mailbox_a = Mailbox::new();
    let mut mailbox_b = Mailbox::new();
    
    // Send a message
    let message = Message::new(
        agent_a,
        agent_b,
        Performative::Inform,
        "Hello, Agent B!"
    );
    
    mailbox_b.send(message).await?;
    
    // Receive message
    if let Some(msg) = mailbox_b.receive().await {
        println!("Agent B received: {}", msg.content());
        println!("From: {}", msg.sender());
        println!("Performative: {:?}", msg.performative());
    }
    
    Ok(())
}
```

### Using the Router
```rust
use agentropic_messaging::prelude::*;

#[tokio::main]
async fn main() -> Result<(), MessagingError> {
    // Create router
    let mut router = Router::new();
    
    // Create and register agents
    let agent_a = AgentId::new();
    let agent_b = AgentId::new();
    
    router.register(agent_a);
    router.register(agent_b);
    
    println!("Registered agents: {}", router.agent_count());
    
    // Create and route message
    let message = Message::new(
        agent_a,
        agent_b,
        Performative::Request,
        "Please send status"
    );
    
    let message_id = *message.id();
    router.route(message).await?;
    
    // Verify delivery
    if router.has_routed(&message_id) {
        println!("Message successfully routed!");
    }
    
    Ok(())
}
```

### Request-Reply Pattern
```rust
use agentropic_messaging::prelude::*;
use agentropic_messaging::protocols::prelude::*;

#[tokio::main]
async fn main() -> Result<(), MessagingError> {
    let requester = AgentId::new();
    let responder = AgentId::new();
    
    // Create request-reply protocol
    let mut protocol = RequestReply::new(requester, responder);
    
    // Requester sends request
    let request = Message::new(
        requester,
        responder,
        Performative::Request,
        "What is the temperature?"
    );
    
    protocol.send_request(request).await?;
    
    // Responder receives and replies
    if let Some(req) = protocol.receive_request().await {
        println!("Received request: {}", req.content());
        
        let reply = Message::new(
            responder,
            requester,
            Performative::Inform,
            "Temperature is 22°C"
        );
        
        protocol.send_reply(reply).await?;
    }
    
    // Requester receives reply
    if let Some(reply) = protocol.receive_reply().await {
        println!("Received reply: {}", reply.content());
    }
    
    Ok(())
}
```

### Different Performatives
```rust
use agentropic_messaging::prelude::*;

fn main() {
    let sender = AgentId::new();
    let receiver = AgentId::new();
    
    // Inform: Share information
    let inform = Message::new(
        sender,
        receiver,
        Performative::Inform,
        "The market is open"
    );
    
    // Request: Ask for action
    let request = Message::new(
        sender,
        receiver,
        Performative::Request,
        "Please buy 100 shares"
    );
    
    // Query: Ask a question
    let query = Message::new(
        sender,
        receiver,
        Performative::Query,
        "What is the current price?"
    );
    
    // Propose: Suggest action
    let propose = Message::new(
        sender,
        receiver,
        Performative::Propose,
        "Let's form a coalition"
    );
    
    // Accept: Agree to proposal
    let accept = Message::new(
        sender,
        receiver,
        Performative::Accept,
        "I accept your proposal"
    );
    
    // CFP: Call for proposals
    let cfp = Message::new(
        sender,
        receiver,
        Performative::CFP,
        "Seeking bids for task execution"
    );
}
```

---

## Architecture

### Message Structure
```rust
pub struct Message {
    id: MessageId,              // Unique UUID-based identifier
    sender: AgentId,            // Sending agent
    receiver: AgentId,          // Receiving agent
    performative: Performative, // Speech act type
    content: String,            // Message payload
}
```

### Message Flow
```
┌──────────┐                ┌──────────┐
│ Agent A  │                │ Agent B  │
│          │                │          │
│ Mailbox  │                │ Mailbox  │
└────┬─────┘                └────┬─────┘
     │                           │
     │    1. Create Message      │
     ├──────────────────────────►│
     │                           │
     │    2. Route via Router    │
     │         (optional)        │
     │                           │
     │    3. Deliver to Mailbox  │
     ├──────────────────────────►│
     │                           │
     │    4. Receive & Process   │
     │                           │
     │◄──────────────────────────┤
     │    5. Send Reply          │
     │                           │
```

### Routing Strategies

**Current (v0.1.0)**:
- **Direct**: Agent-to-agent message delivery
- **Registration**: Agents register with router
- **Delivery Tracking**: Track routed messages

**Future (v0.2.0+)**:
- **Broadcast**: One-to-many delivery
- **Topic-based**: Pub/sub with topics
- **Content-based**: Route by message content
- **Multicast**: Deliver to specific groups

---

## Examples

See the [examples](examples/) directory for complete, runnable examples:

- `simple_messaging.rs` - Basic message passing
- `request_reply.rs` - Request-reply protocol

Run examples:
```bash
cargo run --example simple_messaging
cargo run --example request_reply
```

---

## Related Crates

- **[agentropic-core](../agentropic-core)** - Agent primitives and AgentId
- **[agentropic-cognition](../agentropic-cognition)** - BDI reasoning and planning
- **[agentropic-patterns](../agentropic-patterns)** - Multi-agent coordination patterns
- **[agentropic-runtime](../agentropic-runtime)** - Agent execution engine

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agentropic-messaging).

For guides and tutorials, see the [Agentropic documentation](https://github.com/agentropic/agentropic-docs).

---

## 🎓 References

This crate is inspired by established standards and research:

- **FIPA ACL** (2002) - Foundation for Intelligent Physical Agents Agent Communication Language
- **Speech Act Theory** - Austin (1962) and Searle (1969) - Philosophical foundation for performatives
- **Actor Model** - Hewitt (1973) - Message-passing concurrency model
- **KQML** - Knowledge Query and Manipulation Language - Early agent communication standard

---

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Status

**Active Development** - v0.1.0 released with core message passing, routing, and request-reply protocol.

**Roadmap**:
- v0.2.0: Builder pattern API, topic-based routing, pub/sub
- v0.3.0: Advanced protocols (ContractNet, Auction)
- v0.4.0: Content-based routing, delivery guarantees
- v1.0.0: Stable API with comprehensive protocol library

---

*Part of the [Agentropic](https://github.com/agentropic) ecosystem for agent-oriented programming in Rust.*
