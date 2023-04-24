use std::{env, sync::atomic::AtomicBool, sync::atomic::Ordering};
use leopard::{LeopardBuilder, Leopard};
use pv_recorder::RecorderBuilder;

use crate::dialog_broker::DialogBroker;


pub fn run_recorder(audio_device_index: i32, is_recording: &AtomicBool, dialog_broker: &mut DialogBroker) {
    let access_key = env::var("PICO_ACCESS_KEY").unwrap();

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

    is_recording.store(true, Ordering::SeqCst);

    let mut audio_data = Vec::new();
    recorder.start().expect("Failed to start audio recording");
    while is_recording.load(Ordering::SeqCst) {
        let mut pcm = vec![0; recorder.frame_length()];
        recorder.read(&mut pcm).expect("Failed to read audio frame");
        audio_data.extend_from_slice(&pcm);
    };
    recorder.stop().expect("Failed to stop audio recording");
    let transcript_handle = leopard.process(&audio_data).unwrap();
    dialog_broker.consume_user_message(transcript_handle.transcript)
}