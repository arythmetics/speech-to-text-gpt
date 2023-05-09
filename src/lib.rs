pub mod audio_input;
pub mod recorder;
pub mod gpt_requests;
pub mod dialog_broker;
pub mod utils;
pub mod redis_broker;

use crate::audio_input::{show_audio_devices, get_audio_index_from_user};

pub fn audio_input_setup() -> i32 {
    let mut idx: i32 = -1;
    print!("Hey. Please select an input audio device by typing in its index\n");
    let audio_devices = show_audio_devices();
    get_audio_index_from_user(&mut idx, &audio_devices);

    println!("You selected {}", audio_devices.get(idx as usize).unwrap());
    return idx
}
