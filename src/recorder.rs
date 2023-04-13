use std::{env, sync::atomic::AtomicBool, sync::atomic::Ordering, io::{stdout, Write}};
use cheetah::{CheetahBuilder, Cheetah};
use pv_recorder::RecorderBuilder;
use ctrlc;

pub fn run_recorder(audio_device_index: i32) {
    static RECORDING: AtomicBool = AtomicBool::new(false);
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

    ctrlc::set_handler(|| {
            RECORDING.store(false, Ordering::SeqCst);
        })
        .expect("Unable to setup signal handler");

    RECORDING.store(true, Ordering::SeqCst);

    recorder.start().expect("Failed to start audio recording");
    while RECORDING.load(Ordering::SeqCst) {
        let mut pcm = vec![0; recorder.frame_length()];
        recorder.read(&mut pcm).expect("Failed to read audio frame");

        let partial_transcript = cheetah.process(&pcm).unwrap();
        print!("{}", partial_transcript.transcript);
        stdout().flush().expect("Failed to flush");
        if partial_transcript.is_endpoint {
            let final_transcript = cheetah.flush().unwrap();
            println!("{}", final_transcript.transcript);
        }
    }
    
    println!();
    recorder.stop().expect("Failed to stop audio recording");
}