FROM rust:1.66 as build
WORKDIR /speech-to-text-chatgpt/src/
COPY /src/ .
WORKDIR /speech-to-text-chatgpt/
COPY Cargo.toml .
RUN cargo build --release

FROM ubuntu:20.04
# Install necessary system libraries
RUN apt-get update && \
    apt-get install -y libssl-dev libasound2 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /src/
COPY --from=build /speech-to-text-chatgpt/target/release/speech-to-text-chatgpt .
COPY .env ./
# Copy the dynamic library file to the container
COPY target/release/build/pv_recorder-*/out/lib/linux/x86_64/libpv_recorder.so /usr/lib/
CMD [ "./speech-to-text-chatgpt" ]
