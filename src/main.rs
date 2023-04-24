use dotenv::dotenv;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

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

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();

    loop {
        if let Ok(key_event) = crossterm::event::read() {
            match key_event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers,
                    ..
                }) => {
                    if modifiers == KeyModifiers::empty() {
                        print!("hello");
                        stdout.flush().unwrap();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();

    // while RUNNING.load(Ordering::SeqCst) {
    //     run_recorder(idx, &RECORDING, &mut dialog_broker);
    // }
}
