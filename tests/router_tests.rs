use agentropic_messaging::prelude::*;

#[tokio::test]
async fn router_register_and_send() {
    let router = Router::new();
    let sender_id = AgentId::new();
    let receiver_id = AgentId::new();

    let mut receiver = router.register(receiver_id).unwrap();

    let msg = Message::new(sender_id, receiver_id, Performative::Inform, "test");
    router.send(msg).unwrap();

    let received = receiver.recv().await.unwrap();
    assert_eq!(received.content(), "test");
}

#[tokio::test]
async fn router_send_to_unregistered_fails() {
    let router = Router::new();
    let sender_id = AgentId::new();
    let receiver_id = AgentId::new();

    let msg = Message::new(sender_id, receiver_id, Performative::Inform, "test");
    assert!(router.send(msg).is_err());
}