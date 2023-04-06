use coqui_stt::{Model, Stream};
use std::fs::File;
use std::io::Read;

fn main() {
    let model_path = "assets/models/deepspeech-0.9.3-models.pbmm";
    let scorer_path = "assets/scorer/deepspeech-0.9.3-models.scorer";
    let audio_path = "assets/test-audio/2830-3980-0043.wav";

    // Load the CoquiSTT model
    let mut model = Model::new(model_path).expect("Failed to load the model");
    model.enable_external_scorer(scorer_path).expect("Failed to enable the scorer");

    // Read the audio file
    let mut audio_file = File::open(audio_path).expect("Failed to open the audio file");
    let mut audio_buffer = Vec::new();
    audio_file
        .read_to_end(&mut audio_buffer)
        .expect("Failed to read the audio file");

    // Set up the streaming state
    let mut streaming_state = Stream::from_model(&mut model).expect("Failed to create the streaming state");

    // Feed the audio buffer to the streaming state
    streaming_state.feed_audio(&audio_buffer);

    // Convert the audio to text
    let transcript = streaming_state.finish().expect("Failed to convert speech to text");

    println!("Transcript: {}", transcript);
}
