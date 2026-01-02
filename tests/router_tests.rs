use agentropic_messaging::prelude::*;

#[test]
fn router_register_and_send() {
    let router = Router::new();
    let sender = AgentId::new();
    let receiver = AgentId::new();
    
    let mailbox = router.create_mailbox();
    router.register(receiver, mailbox.clone()).unwrap();
    
    let msg = Message::new(sender, receiver, Performative::Inform, "test");
    router.send(msg).unwrap();
    
    let received = mailbox.try_receive().unwrap();
    assert_eq!(received.content(), "test");
}
