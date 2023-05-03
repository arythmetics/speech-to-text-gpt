use crate::gpt_requests::{post_to_chatgpt, Body, GptResponse};

use log::{error, info};
use reqwest::blocking::{Client, Response};
use reqwest::Error as ReqwestError;
use std::{env::var, io::Write};
use std::error::Error as StdError;
use std::fmt;

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
        // Remove
        self.user_content = String::from("Hi ChatGPT, how are you doing today?");
        print!("\n");
        print!("===========================\n");
        print!("USER: {}\n", self.user_content);
        std::io::stdout().flush().unwrap()
    }

    fn consume_chatgpt_message(&mut self, chatgpt_content: GptResponse) {
        // Unsafe function. Need to add error handling around a blank gpt response
        self.chatgpt_content = chatgpt_content.choices.get(0).unwrap().message.content.clone();
        print!("\n");
        print!("CHATGPT: {}\n", self.chatgpt_content);
        print!("===========================\n");
        std::io::stdout().flush().unwrap()
    }

    pub fn communicate_to_chatgpt(&mut self) {
        let res = post_to_chatgpt(&self.client, Body::new(&self.user_content), &var("OPENAI_API_KEY").unwrap());

        match res {
            Ok(r) => {
                let deserialzied_res = deserialize_chatgpt_response(r);
                match deserialzied_res {
                    Ok(chatgpt_content) => self.consume_chatgpt_message(chatgpt_content),
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
