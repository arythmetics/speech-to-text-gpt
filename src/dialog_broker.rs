use crate::gpt_requests::{post_to_chatgpt, Body, GptResponse};
use crate::redis_broker::{publish_messages, establish_connection};

use log::{error, info};
use reqwest::blocking::{Client, Response};
use reqwest::Error as ReqwestError;
use std::{env::var, io::Write};
use std::error::Error as StdError;
use std::fmt;
use async_std::sync::{Arc, Mutex};
use redis::aio::Connection;

#[derive(Default, Clone)]
pub struct DialogBroker {
    pub chatgpt_client: Client,
    pub user_content: String,
    pub chatgpt_content: String,
    pub redis_connection: Option<Arc<Mutex<Connection>>>,
}

impl DialogBroker {
    pub async fn init() -> DialogBroker {
        let redis_connection = establish_connection().await;
        let dialog_broker = DialogBroker {
            chatgpt_client: Client::new(),
            redis_connection: Some(Arc::new(Mutex::new(redis_connection))),
            ..Default::default()
        };
        dialog_broker
    }

    pub async fn consume_user_message(&mut self, user_content: String) {
        self.user_content = user_content;
        self.user_content = String::from("Hey ChatGPT, How are you?");
        if let Some(redis_connection) = &self.redis_connection {
            let mut connection = redis_connection.lock().await;
            publish_messages(&mut connection, self.user_content.clone()).await;
        }
    }

    async fn consume_chatgpt_message(&mut self, chatgpt_content: GptResponse) {
        // Unsafe function. Need to add error handling around a blank gpt response
        self.chatgpt_content = chatgpt_content.choices.get(0).unwrap().message.content.clone();
        if let Some(redis_connection) = &self.redis_connection {
            let mut connection = redis_connection.lock().await;
            publish_messages(&mut connection, self.chatgpt_content.clone()).await;
        }
    }

    pub async fn communicate_to_chatgpt(&mut self, payload: String) {
        let res = post_to_chatgpt(&self.chatgpt_client, Body::new(&payload), &var("OPENAI_API_KEY").unwrap());

        match res {
            Ok(r) => {
                let deserialzied_res = deserialize_chatgpt_response(r);
                match deserialzied_res {
                    Ok(chatgpt_content) => self.consume_chatgpt_message(chatgpt_content).await,
                    Err(e) => error!("Unable to deserialize message from OpenAI API: {}", e)        
                }
            }
            Err(e) => error!("Unable to receive message from OpenAI API: {}", e),
        };
    }
}

pub fn deserialize_chatgpt_response(chatgpt_response: Response) -> Result<GptResponse, ChatGptError> {
    // Log the raw response
    let raw_res = chatgpt_response.text().unwrap();
    info!("Raw response: {}", raw_res);
    std::io::stdout().flush().unwrap();

    // Deserialize the response
    match serde_json::from_str::<GptResponse>(&raw_res) {
        Ok(deserialized_gpt_response) => Ok(deserialized_gpt_response),
        Err(e) => {
            error!("Unable to deserialize response: {:?}", e);
            Err(ChatGptError::from(e))
        }
    }
}

#[derive(Debug)]
pub enum ChatGptError {
    ReqwestError(ReqwestError),
    DeserializeError(serde_json::Error),
}

impl fmt::Display for ChatGptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatGptError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            ChatGptError::DeserializeError(e) => write!(f, "Deserialize error: {}", e),
        }
    }
}

impl StdError for ChatGptError {}

impl From<ReqwestError> for ChatGptError {
    fn from(error: ReqwestError) -> Self {
        ChatGptError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for ChatGptError {
    fn from(error: serde_json::Error) -> Self {
        ChatGptError::DeserializeError(error)
    }
}
