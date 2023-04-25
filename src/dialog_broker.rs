use crate::gpt_requests::{post_to_chatgpt, Body, GptResponse};

use reqwest::{Client, Response, Error as ReqwestError};
use std::{env::var, io::Write};
use log::error;

#[derive(Default, Clone)]
pub struct DialogBroker {
    pub client: Client,
    pub user_content: String,
    pub chatgpt_content: String,
}

impl DialogBroker {
    pub fn init() -> DialogBroker {
        let dialog_broker = DialogBroker {
            client: Client::new(),
            ..Default::default()
        };
        dialog_broker
    }

    pub fn consume_user_message(&mut self, user_content: String) {
        self.user_content = user_content;
        print!("\n");
        print!("===========================\n");
        print!("USER: {}\n", self.user_content);
        std::io::stdout().flush().unwrap()
    }

    fn consume_chatgpt_message(&mut self, chatgpt_content: GptResponse) {
        // Unsafe function. Need to add error handling around a blank gpt response
        self.chatgpt_content = chatgpt_content.choices.get(0).unwrap().message.content.clone();
    }

    async fn deserialize_chatgpt_response(chatgpt_response: Result<Response, ReqwestError>) -> Result<GptResponse, ReqwestError> {
        match chatgpt_response {
            Ok(res) => {
                let deserialied_gpt_response: GptResponse = res.json().await?;
                Ok(deserialied_gpt_response)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn communicate_to_chatgpt(&mut self) {
        let res = post_to_chatgpt(&self.client, Body::new(&self.user_content), &var("OPENAI_API_KEY").unwrap()).await;
        let deserialzied_res = Self::deserialize_chatgpt_response(res).await;

        match deserialzied_res {
            Ok(chatgpt_content) => self.consume_chatgpt_message(chatgpt_content),
            Err(e) => error!("Unable to receive message from OpenAI API: {}", e)
        }
    }
}
