use crate::gpt_requests::{post_to_chatgpt, Body};

use reqwest::{Client, Response};
use std::env::var;

#[derive(Default)]
struct DialogBroker {
    client: Client,
    user_content: String,
    chatgpt_content: String,
}

impl DialogBroker {
    fn init() -> DialogBroker {
        let dialog_broker = DialogBroker {
            client: Client::new(),
            ..Default::default()
        };
        dialog_broker
    }

    fn consume_user_message(&mut self, user_content: String) {
        self.user_content = user_content
    }

    fn deserialize_chatgpt_response(chatgpt_response: Response) {

    }

    async fn communicate_to_chatgpt(self) {
        let res = post_to_chatgpt(&self.client, Body::new(&self.user_content), &var("OPENAI_API_KEY").unwrap()).await;
        // Need to deserialize gpt response
    }
}
