# agentropic-messaging

[![Crates.io](https://img.shields.io/crates/v/agentropic-messaging.svg)](https://crates.io/crates/agentropic-messaging)
[![Documentation](https://docs.rs/agentropic-messaging/badge.svg)](https://docs.rs/agentropic-messaging)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)![CI](https://github.com/pycogram/agentropic-messaging/workflows/CI/badge.svg)

**Communication protocols, message routing, and Agent Communication Language (ACL) for multi-agent systems.**

`agentropic-messaging` provides the infrastructure for agents to communicate, coordinate, and collaborate. It implements message passing semantics, routing mechanisms, and standardized communication protocols that enable sophisticated multi-agent interactions.

---

## 🎯 Purpose

This crate provides:

- **Message Protocols**: Structured message formats and semantics
- **Agent Communication Language (ACL)**: Standardized speech acts and performatives
- **Message Routing**: Intelligent delivery of messages between agents
- **Communication Patterns**: Common interaction protocols (request-reply, broadcast, etc.)

---

## 🧩 Core Concepts

### Agent Messages

Messages are the fundamental unit of agent communication:
```rust
use agentropic_messaging::{Message, MessageBuilder, Performative};
use agentropic_core::AgentId;

let message = MessageBuilder::new()
    .sender(sender_id)
    .receiver(receiver_id)
    .performative(Performative::Request)
    .content("Get market data")
    .build();
```

### Performatives (Speech Acts)

Based on speech act theory, performatives define the intent of a message:

- **Inform** - Share information
- **Request** - Ask for an action or information
- **Query** - Ask a question
- **Propose** - Suggest a course of action
- **Accept** - Agree to a proposal
- **Reject** - Decline a proposal
- **Confirm** - Verify information
- **Disconfirm** - Contradict information
- **Subscribe** - Request ongoing updates
- **CFP (Call for Proposals)** - Solicit offers
- **Refuse** - Decline to perform an action
```rust
use agentropic_messaging::Performative;

// Request information
let query = Message::new(
    sender,
    receiver,
    Performative::Query,
    "What is the current temperature?"
);

// Inform with data
let response = Message::new(
    receiver,
    sender,
    Performative::Inform,
    "The current temperature is 22°C"
);
```

### Message Routing

The router delivers messages to appropriate recipients:
```rust
use agentropic_messaging::{Router, RoutingStrategy};

let mut router = Router::new();

// Register an agent
router.register_agent(agent_id, mailbox);

// Send a message
router.send(message).await?;

// Broadcast to multiple agents
router.broadcast(message, agent_ids).await?;
```

### Communication Protocols

Higher-level interaction patterns:
```rust
use agentropic_messaging::protocols::{RequestReply, ContractNet};

// Request-Reply pattern
let reply = RequestReply::execute(
    sender,
    receiver,
    "Perform calculation",
    timeout
).await?;

// Contract Net Protocol (CNP)
let bids = ContractNet::call_for_proposals(
    manager,
    participants,
    task_description
).await?;
```

---

## 📦 What's Included

### Core Types

- `Message` - The fundamental message structure
- `MessageBuilder` - Fluent API for message construction
- `Performative` - Speech act types (Inform, Request, Query, etc.)
- `MessageContent` - Typed message payloads

### Routing

- `Router` - Central message routing engine
- `Mailbox` - Agent message queue
- `RoutingStrategy` - Direct, broadcast, multicast routing
- `MessageFilter` - Content-based routing and filtering

### Protocols

- `RequestReply` - Synchronous request-response
- `Subscribe` - Pub/sub patterns
- `ContractNet` - Task allocation through bidding
- `AuctionProtocol` - Market-based coordination

### ACL Support

- FIPA-inspired message structure
- Conversation tracking with `conversation_id`
- Reply management with `in_reply_to`
- Protocol specification fields

---

## 🚀 Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agentropic-messaging = "0.1.0"
agentropic-core = "0.1.0"
```

### Basic Message Passing
```rust
use agentropic_messaging::{Message, MessageBuilder, Performative, Router};
use agentropic_core::AgentId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create router
    let mut router = Router::new();
    
    // Create agents
    let agent_a = AgentId::new();
    let agent_b = AgentId::new();
    
    // Register agents with mailboxes
    let mailbox_a = router.create_mailbox();
    let mailbox_b = router.create_mailbox();
    router.register_agent(agent_a, mailbox_a.clone())?;
    router.register_agent(agent_b, mailbox_b.clone())?;
    
    // Send a message
    let message = MessageBuilder::new()
        .sender(agent_a)
        .receiver(agent_b)
        .performative(Performative::Request)
        .content("Hello, Agent B!")
        .build();
    
    router.send(message).await?;
    
    // Receive message
    if let Some(msg) = mailbox_b.receive().await {
        println!("Received: {:?}", msg.content());
    }
    
    Ok(())
}
```

### Request-Reply Pattern
```rust
use agentropic_messaging::{MessageBuilder, Performative};
use std::time::Duration;

async fn request_reply_example(
    router: &Router,
    requester: AgentId,
    responder: AgentId
) -> Result<String, Box<dyn std::error::Error>> {
    // Send request
    let request = MessageBuilder::new()
        .sender(requester)
        .receiver(responder)
        .performative(Performative::Request)
        .content("Get status")
        .conversation_id("conv-123")
        .build();
    
    router.send(request).await?;
    
    // Wait for reply
    let reply = router.wait_for_reply(
        requester,
        "conv-123",
        Duration::from_secs(5)
    ).await?;
    
    Ok(reply.content().to_string())
}
```

### Pub/Sub Pattern
```rust
use agentropic_messaging::{Topic, Subscription};

// Create a topic
let topic = Topic::new("market.prices");

// Publisher
async fn publish(router: &Router, publisher_id: AgentId, data: &str) {
    let message = MessageBuilder::new()
        .sender(publisher_id)
        .performative(Performative::Inform)
        .topic("market.prices")
        .content(data)
        .build();
    
    router.publish(message).await?;
}

// Subscriber
async fn subscribe(router: &Router, subscriber_id: AgentId) {
    router.subscribe(subscriber_id, "market.prices").await?;
    
    while let Some(msg) = router.receive(subscriber_id).await {
        println!("Received update: {}", msg.content());
    }
}
```

---

## 🏗️ Architecture

### Message Structure
```rust
pub struct Message {
    pub id: MessageId,              // Unique message identifier
    pub sender: AgentId,             // Sender agent
    pub receiver: AgentId,           // Receiver agent (or topic)
    pub performative: Performative,  // Speech act type
    pub content: MessageContent,     // Message payload
    pub conversation_id: Option<String>, // Conversation tracking
    pub in_reply_to: Option<MessageId>,  // Reply reference
    pub protocol: Option<String>,    // Protocol identifier
    pub timestamp: SystemTime,       // Message creation time
}
```

### Routing Strategies

- **Direct**: One-to-one message delivery
- **Broadcast**: One-to-many to all agents
- **Multicast**: One-to-many to specific agents
- **Topic-based**: Pub/sub with topic routing
- **Content-based**: Route by message content

### Delivery Guarantees

- **At-most-once**: Fire and forget
- **At-least-once**: Retry until acknowledged
- **Exactly-once**: Deduplication and acknowledgment

---

## 🔗 Related Crates

- **[agentropic-core](../agentropic-core)** - Agent primitives and traits
- **[agentropic-cognition](../agentropic-cognition)** - Reasoning and planning
- **[agentropic-patterns](../agentropic-patterns)** - Multi-agent system patterns
- **[agentropic-runtime](../agentropic-runtime)** - Agent execution engine

---

## 📚 Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agentropic-messaging).

For guides and tutorials, see [agentropic-docs](https://github.com/agentropic/agentropic-docs).

---

## 🎓 References

This crate is inspired by:

- **FIPA ACL** - Foundation for Intelligent Physical Agents Agent Communication Language
- **Speech Act Theory** - Philosophical foundation for performatives
- **Actor Model** - Message-passing concurrency

---

## 🤝 Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md).

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## 🌟 Status

**Active Development** - This crate is under active development. APIs may change before 1.0 release.

---

*Part of the [Agentropic](https://github.com/agentropic) ecosystem for agent-oriented programming in Rust.*