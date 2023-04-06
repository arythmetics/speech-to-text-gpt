use cheetah::{CheetahBuilder, Cheetah};
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let audio_path = "/Users/arya/projects/speech-to-text-chatgpt/assets/test-audiio/2830-3980-0043.wav";

    let access_key = env::var("PICO_ACCESS_KEY").unwrap();

    // Create a Picovoice instance with the Cheetah ASR model
    let cheetah: Cheetah = CheetahBuilder::new()
        .access_key(access_key)
        .init()
        .expect("Unable to create Cheetah");


    // Read the audio file
    fn next_audio_frame(audio_path: &str) -> Vec<i16> {
        let mut reader = hound::WavReader::open(audio_path).expect("Failed to open the audio file");
        let audio_buffer: Vec<i16> = reader.samples::<i16>().map(Result::unwrap).collect();
        return audio_buffer
    }

    if let Ok(cheetah_transcript) = cheetah.process(&next_audio_frame(&audio_path)) {
        println!("{}", cheetah_transcript.transcript);
        if cheetah_transcript.is_endpoint {
          if let Ok(cheetah_transcript) = cheetah.flush() {
            println!("{}", cheetah_transcript.transcript)
          }
        }
      }
}
