use crate::dialog_broker::DialogBroker;

use std::thread;
use std::{env, sync::atomic::AtomicBool, sync::atomic::Ordering};
use leopard::{LeopardBuilder, Leopard};
use pv_recorder::RecorderBuilder;
use std::io::stdout;
use std::io::Write;
use std::io;


pub fn run_recorder(audio_device_index: i32, dialog_broker: &mut DialogBroker) {
    let mut input = String::new();
    let access_key = env::var("PICO_ACCESS_KEY").unwrap();
    static RECORDING: AtomicBool = AtomicBool::new(false);

    // Leopard, for translating audio files
    let leopard: Leopard = LeopardBuilder::new()
        .enable_automatic_punctuation(true)
        .access_key(access_key)
        .init()
        .expect("Unable to create Leopard");

    let recorder = RecorderBuilder::new()
        .device_index(audio_device_index)
        .frame_length(512)
        .init()
        .expect("Failed to initialize pvrecorder");

    print!("\n>>> Press 'Enter' to start recording: ");
    stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    RECORDING.store(true, Ordering::SeqCst);

    let mut audio_data = Vec::new();

    let leopard = leopard.clone();
    let recorder = recorder.clone();

    let transcript_handle = thread::spawn(move || {
        recorder.start().expect("Failed to start audio recording");
        while RECORDING.load(Ordering::SeqCst) {
            let mut pcm = vec![0; recorder.frame_length()];
            recorder.read(&mut pcm).expect("Failed to read audio frame");
            audio_data.extend_from_slice(&pcm);
        }
        recorder.stop().expect("Failed to stop audio recording");
        leopard.process(&audio_data).unwrap()
    });

    print!(">>> Recording ... Speak Clearly ... Press 'Enter' to stop: \n");
        stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        RECORDING.store(false, Ordering::SeqCst);

    let leopard_transcript = transcript_handle.join().unwrap();
    dialog_broker.consume_user_message(leopard_transcript.transcript)
}
