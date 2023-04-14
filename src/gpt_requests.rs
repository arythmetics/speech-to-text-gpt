use serde::{Serialize, Deserialize};
use reqwest::{Client, Error as ReqwestError, Response as ReqwestResponse};
use std::env::var;

#[derive(Serialize, Deserialize)]
struct Messages {
    role: String,
    content: String
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    model: String,
    messages: Vec<Messages>,
    pub temperature: u8,
}

impl Body {
    pub fn new(content: &String) -> Body {
        Body {
            model: String::from("gpt-3.5-turbo"),
            messages: vec![Messages { role: String::from("user"), content: content.to_string() }],
            temperature: 10
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GptResponse {
    pub choices: Vec<ChoiceObject>,
    created: u32,
    id: String,
    model: String,
    object: String,
    usage: UsageObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceObject {
finish_reason: String,
index: u32,
pub message: MessageObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageObject {
    pub content: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsageObject {
    completion_tokens: u16,
    prompt_tokens: u16,
    total_tokens: u16,
}

pub async fn post_to_chatgpt(client: &Client, body: Body, token: &String) -> Result<ReqwestResponse, ReqwestError> {
    let res = client.post(var("CHAT_GPT_API").unwrap())
        .json(&body)
        .bearer_auth(token)
        .send()
        .await?;
    Ok(res)
}
