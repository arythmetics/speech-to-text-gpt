use serde::{Serialize, Deserialize};
use reqwest::{Client, Error as ReqwestError, Response as ReqwestResponse};

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
    pub fn new(content: String) -> Body {
        Body {
            model: String::from("gpt-3.5-turbo"),
            messages: vec![Messages { role: String::from("user"), content: content }],
            temperature: 10
        }
    }
}

pub async fn post_to_chatgpt(client: &Client, body: &Body, token: &String) -> Result<ReqwestResponse, ReqwestError> {
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .json(body)
        .bearer_auth(token)
        .send()
        .await?;
    Ok(res)
}