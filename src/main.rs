use dotenv::dotenv;
use std::{sync::atomic::{AtomicBool, Ordering}};
use crossterm::{
    execute,
    terminal::EnterAlternateScreen,
};
use std::io::stdout;

use speech_to_text_chatgpt::{
    dialog_broker::DialogBroker, 
    utils::close_program, 
    audio_input_setup, 
    recorder::run_recorder
};

fn main() {
    dotenv().ok();
    
    ctrlc::set_handler(|| {
        close_program(&RUNNING)
    })
    .expect("Unable to setup signal handler");

    static RUNNING: AtomicBool = AtomicBool::new(true);
    let mut dialog_broker: DialogBroker = DialogBroker::init();
    let idx = audio_input_setup();
    
    execute!(stdout(), EnterAlternateScreen).unwrap();

    print!("Press CTRL + C to exit the program\n");

    while RUNNING.load(Ordering::SeqCst) {
        let audio_device_index = idx.clone();
        run_recorder(audio_device_index, &mut dialog_broker)
    }
}
