use cheetah::{CheetahBuilder, Cheetah};
use pv_recorder::RecorderBuilder;
use std::{env};
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use dotenv::dotenv;

static RECORDING: AtomicBool = AtomicBool::new(false);

fn main() {
    dotenv().ok();

    let mut idx: i32 = -1;
    print!("Hey. Please select an input audio device by typing in its index\n");
    let audio_devices = show_audio_devices();
    get_audio_index_from_user(&mut idx, &audio_devices);

    println!("You selected {}", audio_devices.get(idx as usize).unwrap());

    run_recorder(idx)
}

fn run_recorder(audio_device_index: i32) {
    let access_key = env::var("PICO_ACCESS_KEY").unwrap();

    // Cheetah, for Real Time
    let cheetah: Cheetah = CheetahBuilder::new()
        .enable_automatic_punctuation(true)
        .endpoint_duration_sec(2.0)
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

fn show_audio_devices() -> Vec<String> {
    let audio_devices = RecorderBuilder::new()
        .init()
        .expect("Failed to initialize pvrecorder")
        .get_audio_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (idx, device) in audio_devices.iter().enumerate() {
                println!("index: {}, device name: {:?}", idx, device);
            }
        return audio_devices
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
}

fn get_audio_index_from_user(idx: &mut i32, audio_devices: &Vec<String>) {
    loop {
        stdout().flush().unwrap();
        let number_of_audio_devices: i32 = audio_devices.len().try_into().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.\n");

        match input.trim().parse::<i32>() {
            Ok(value) => {
                if (value >= number_of_audio_devices) || (value < 0) {
                    println!("\nPick one of the options below. Please try again.");
                } else {
                    *idx = value;
                    break;
                } 
            }
            Err(_) => {
                println!("\nInput a valid integer. Please try again.");
            }
        }
    }
}
