use pv_recorder::RecorderBuilder;
use std::io::{stdin, stdout, Write};

pub fn show_audio_devices() -> Vec<String> {
    let audio_devices = RecorderBuilder::new().get_audio_devices();
        
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

pub fn get_audio_index_from_user(idx: &mut i32, audio_devices: &Vec<String>) {
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