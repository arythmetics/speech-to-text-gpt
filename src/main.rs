use speech_to_text_chatgpt::{
    audio_input::*, 
    recorder::*
};

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let mut idx: i32 = -1;
    print!("Hey. Please select an input audio device by typing in its index\n");
    let audio_devices = show_audio_devices();
    get_audio_index_from_user(&mut idx, &audio_devices);

    println!("You selected {}", audio_devices.get(idx as usize).unwrap());

    run_recorder(idx)
}
