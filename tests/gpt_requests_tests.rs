use speech_to_text_chatgpt::{
    gpt_requests::{post_to_chatgpt, Body},
};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use tokio::runtime::Runtime;
use serde_json::Value;

#[test]
fn test_post_to_chatgpt() {
    dotenv().ok();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = Client::new();

        let test_message = String::from("say this is a test");

        let mut body = Body::new(test_message);
        body.temperature = 0;

        let token = env::var("OPENAI_API_KEY").unwrap();

        let res = post_to_chatgpt(&client, &body, &token).await;
        assert!(res.is_ok());
        let content_from_gpt:Result<Value, reqwest::Error> = res.unwrap().json().await;
        println!("{:#?}", content_from_gpt);
    });
}