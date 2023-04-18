use std::sync::atomic::{AtomicBool, Ordering};

use speech_to_text_chatgpt::{
    recorder_setup
};

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    static RUNNING: AtomicBool = AtomicBool::new(true);
    static RECORDING: AtomicBool = AtomicBool::new(false);

    ctrlc::set_handler(|| {
        RUNNING.store(false, Ordering::SeqCst)
    })
    .expect("Unable to setup signal handler");

    while RUNNING.load(Ordering::SeqCst) {
        
    }

    // recorder_setup(&RECORDING)
}
