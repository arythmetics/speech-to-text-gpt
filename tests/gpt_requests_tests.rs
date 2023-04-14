use speech_to_text_chatgpt::{
    gpt_requests::{post_to_chatgpt, Body, GptResponse, MessageObject},
};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use tokio::runtime::Runtime;

#[test]
fn test_post_to_chatgpt() {
    dotenv().ok();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = Client::new();

        let test_message = String::from("say this is a test");

        let mut body = Body::new(&test_message);
        body.temperature = 0;

        let token = env::var("OPENAI_API_KEY").unwrap();

        let res = post_to_chatgpt(&client, body, &token).await;
        assert!(res.is_ok());

        let content_from_gpt:Result<GptResponse, reqwest::Error> = res.unwrap().json().await;
        assert!(content_from_gpt.is_ok());

        let gpt_response: GptResponse = content_from_gpt.unwrap();
        assert!(gpt_response.choices.len() > 0);

        let gpt_message: &MessageObject = &gpt_response.choices.get(0).unwrap().message;
        assert_eq!(gpt_message.content, String::from("This is a test."))
    });
}