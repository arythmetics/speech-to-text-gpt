use serde::{Serialize, Deserialize};
use reqwest::{Client, Error as ReqwestError};

#[derive(Serialize, Deserialize)]
struct Messages {
    role: String,
    content: String
}

#[derive(Serialize, Deserialize)]
struct Body {
    model: String,
    messages: Vec<Messages>,
}

impl Body {
    fn new(content: String) -> Body {
        Body {
            model: String::from("gpt-3.5-turbo"),
            messages: vec![Messages { role: String::from("user"), content: content }]
        }
    }
}

async fn post_to_chatgpt(client: &Client, body: &Body) -> Result<(), ReqwestError> {
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .form(body)
        .send()
        .await?;
    Ok(())
}