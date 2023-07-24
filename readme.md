# Speech to GPT (s2gpt)

s2gpt is a rust based CLI application for communicating with ChatGPT using your voice. Voice transcription is specifically done locally to avoid having .wav files of people's voices in far flung server farms.

## Running locally
Setting this up to run locally requires a .env file, a redis docker container, and Rust. *Make sure to run this on a machine that has access to your mic*

1. Create the .env file and add the following variables: 
    PICO_ACCESS_KEY=[value]
    OPENAI_API_KEY=[value]
    CHAT_GPT_API=[value]
    REDIS_HOST=[value]
2. Run the redis-stack-server with the following docker command
    `docker run -d --name redis-stack-server -p 6379:6379 redis/redis-stack-server:latest`
3. Install Rust. If you're on Mac or Linux then run:
    `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

Once all this is set up you just need to run this command to run it in the terminal:
    `cargo run`

## Road Map
- Full Dockerization. The reason this application could not be dockerized form the start is that Docker does not have great support for accessing audio input on machine. What would likely be the best way to run this as a cli application that is mostly dockerized is to run the recorder locally, then the dialog broker and redis in their own containers. The recorder would then send the transcription to redis and the dialog broker would communicate between chatgpt, the user, and redis. These three services are already separated to be running on their own threads.
- Speech to Text Algorithm. This application is using the Picovoice default algorithm. Picovoice is great because it does all transcription locally and it's free for skunkwork projects like this. The default model though is... okay. It makes mistakes more than I'd like. Picovoice allows you to make adjustments though so there's some unexplored experimentation that I've yet to do. Also my mic is foul and is likely to source of most of my transcription issues.
- Background noise muting. Background noise confuses the transcription process. I'd like to expand the Picovoice model to edit the .wav file for background noise.
- Test point for Khalil
