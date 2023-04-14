use speech_to_text_chatgpt::{
    recorder_setup
};

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    recorder_setup()
}
