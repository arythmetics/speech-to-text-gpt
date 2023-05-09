use dotenv::dotenv;
use std::{sync::atomic::{AtomicBool, Ordering}};
use crossterm::{
    execute,
    terminal::EnterAlternateScreen,
};
use std::io::stdout;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

use speech_to_text_chatgpt::{
    dialog_broker::DialogBroker, 
    utils::close_program, 
    audio_input_setup, 
    recorder::run_recorder,
    redis_broker::run
};

// fn main() {
//     dotenv().ok();
//     let pattern = "{d(%Y-%m-%d %H:%M:%S)} {l} {M}: {m}{n}";
//     let file_appender = FileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(pattern)))
//         .build("log/output.log")
//         .unwrap();

//     let config = Config::builder()
//         .appender(Appender::builder().build("file", Box::new(file_appender)))
//         .logger(Logger::builder().build("app", LevelFilter::Info))
//         .build(Root::builder().appender("file").build(LevelFilter::Info))
//         .unwrap();

//     log4rs::init_config(config).unwrap();
    
//     ctrlc::set_handler(|| {
//         close_program(&RUNNING)
//     })
//     .expect("Unable to setup signal handler");

//     static RUNNING: AtomicBool = AtomicBool::new(true);
//     let mut dialog_broker: DialogBroker = DialogBroker::init();
//     let idx = audio_input_setup();
    
//     execute!(stdout(), EnterAlternateScreen).unwrap();

//     print!("Press CTRL + C to exit the program\n");

//     while RUNNING.load(Ordering::SeqCst) {
//         let audio_device_index = idx.clone();
//         run_recorder(audio_device_index, &mut dialog_broker);
//         dialog_broker.communicate_to_chatgpt()
//     }
// }

#[async_std::main]
async fn main() {
    dotenv().ok();
    run().await;
}
