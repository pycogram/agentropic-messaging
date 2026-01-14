use agentropic_messaging::prelude::*;

#[test]
fn create_message() {
    let sender = AgentId::new();
    let receiver = AgentId::new();

    let msg = Message::new(sender, receiver, Performative::Inform, "test");

    assert_eq!(msg.sender(), sender);
    assert_eq!(msg.receiver(), receiver);
    assert_eq!(msg.performative(), Performative::Inform);
    assert_eq!(msg.content(), "test");
}

#[test]
fn message_builder() {
    let sender = AgentId::new();
    let receiver = AgentId::new();

    let msg = MessageBuilder::new()
        .sender(sender)
        .receiver(receiver)
        .performative(Performative::Request)
        .content("Hello")
        .build();

    assert_eq!(msg.sender(), sender);
    assert_eq!(msg.content(), "Hello");
}
