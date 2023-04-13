#[derive(Default)]
struct MessageBroker {
    user_content: String,
    chatgpt_content: String,
}

impl MessageBroker {
    fn init() -> MessageBroker {
        let message_broker = MessageBroker {
            ..Default::default()
        };
        message_broker
    }
}