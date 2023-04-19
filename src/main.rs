use dotenv::dotenv;
use std::sync::atomic::{AtomicBool, Ordering};

use speech_to_text_chatgpt::{
    dialog_broker::DialogBroker, 
    utils::close_program, audio_input_setup, 
    recorder::run_recorder
};

fn main() {
    dotenv().ok();
    static RUNNING: AtomicBool = AtomicBool::new(true);
    static RECORDING: AtomicBool = AtomicBool::new(false);

    let mut dialog_broker: DialogBroker = DialogBroker::init();

    ctrlc::set_handler(|| {
        close_program(&RECORDING, &RUNNING)
    })
    .expect("Unable to setup signal handler");

    let idx = audio_input_setup();

    while RUNNING.load(Ordering::SeqCst) {
        run_recorder(idx, &RECORDING, &mut dialog_broker);
        println!("{}", dialog_broker.user_content)
    }
}
