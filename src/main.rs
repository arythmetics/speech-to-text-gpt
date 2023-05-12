use async_std::channel;
use dotenv::dotenv;
use std::{sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};
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
    redis_broker::run_redis_listener
};

#[async_std::main]
async fn main() {
    dotenv().ok();
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} {l} {M}: {m}{n}";
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .logger(Logger::builder().build("app", LevelFilter::Info))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
    
    ctrlc::set_handler(|| {
        close_program(&RUNNING)
    })
    .expect("Unable to setup signal handler");

    static RUNNING: AtomicBool = AtomicBool::new(true);
    let mut dialog_broker: DialogBroker = DialogBroker::init().await;
    let idx = audio_input_setup();
    
    execute!(stdout(), EnterAlternateScreen).unwrap();

    print!("Press CTRL + C to exit the program\n");

    // This creates a channel to pass messages between the redis listener thread and the main thread
    let (sender, receiver) = channel::unbounded::<String>();

    // Run the listener on a separate thread
    run_redis_listener(sender).await;

    while RUNNING.load(Ordering::SeqCst) {
        let audio_device_index = idx.clone();

        // Initiate recording on current thread
        let user_transcription = run_recorder(audio_device_index).await;

        // Publish user transcription to redis
        dialog_broker.consume_user_message(user_transcription).await;
        thread::sleep(Duration::from_secs(1));

        // Pull user recording from the listener thread and send to ChatGPT
        if let Ok(payload) = receiver.try_recv() {
            dialog_broker.communicate_to_chatgpt(payload).await
        } else {
            print!("{:#?}", receiver.try_recv())
        }
    }
}
