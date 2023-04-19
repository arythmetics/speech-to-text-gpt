use std::{env, sync::atomic::AtomicBool, sync::atomic::Ordering, io::{stdout, Write}};
use cheetah::{CheetahBuilder, Cheetah};
use pv_recorder::RecorderBuilder;

use crate::dialog_broker::DialogBroker;


pub fn run_recorder(audio_device_index: i32, is_recording: &AtomicBool, dialog_broker: &mut DialogBroker) {
    let access_key = env::var("PICO_ACCESS_KEY").unwrap();

    // Cheetah, for Real Time
    let cheetah: Cheetah = CheetahBuilder::new()
        .enable_automatic_punctuation(true)
        .endpoint_duration_sec(1.5)
        .access_key(access_key)
        .init()
        .expect("Unable to create Cheetah");

    let recorder = RecorderBuilder::new()
        .device_index(audio_device_index)
        .frame_length(cheetah.frame_length() as i32)
        .init()
        .expect("Failed to initialize pvrecorder");

    is_recording.store(true, Ordering::SeqCst);

    recorder.start().expect("Failed to start audio recording");
    while is_recording.load(Ordering::SeqCst) {
        let mut pcm = vec![0; recorder.frame_length()];
        recorder.read(&mut pcm).expect("Failed to read audio frame");

        let partial_transcript = cheetah.process(&pcm).unwrap();
        // print!("{}", partial_transcript.transcript);
        // stdout().flush().expect("Failed to flush");
        if partial_transcript.is_endpoint {
            is_recording.store(false, Ordering::SeqCst);
            let final_transcript = cheetah.flush().unwrap();
            dialog_broker.consume_user_message(final_transcript.transcript);
        }
    }
    
    println!();
    recorder.stop().expect("Failed to stop audio recording");
}