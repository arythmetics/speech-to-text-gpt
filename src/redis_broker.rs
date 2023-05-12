use std::{env, io::Write};
use async_std::channel;
use futures::StreamExt;
use redis::{AsyncCommands, aio::Connection, aio::PubSub};

pub async fn publish_messages(con: &mut Connection, message: String) {
    let channel_name = "my_channel";

    // Publish the message to the channel
    let _: () = con.publish(channel_name, &message).await.unwrap();
}

async fn subscribe_to_messages(mut pubsub: PubSub, sender: channel::Sender<String>) {
    let channel_name = "my_channel";

    let mut user_source: bool = true;

    // Subscribe to the channel
    pubsub.subscribe(channel_name).await.unwrap();

    // Listen for messages
    while let Some(msg) = pubsub.on_message().next().await {
        let payload: String = msg.get_payload().unwrap();
        if user_source {
            print!("\n");
            print!("===========================\n");
            print!("USER: {}\n", payload);
            std::io::stdout().flush().unwrap();
            user_source = false;

            // Send the payload to the main thread
            if let Err(err) = sender.send(payload).await {
                print!("Error sending payload to main thread: {}", err);
                std::io::stdout().flush().unwrap();
                user_source = true;
            }
        } else {
            print!("\n");
            print!("CHATGPT: {}\n", payload);
            print!("===========================\n");
            std::io::stdout().flush().unwrap();
            user_source = true;
        }
    }
}

pub async fn establish_connection() -> Connection {
    let client = redis::Client::open(env::var("REDIS_HOST").unwrap()).unwrap();
    let con = client.get_async_std_connection().await.unwrap();
    return con
}

pub async fn run_redis_listener(sender: channel::Sender<String>) {
    let redis_host: String = env::var("REDIS_HOST").unwrap();
    let client = redis::Client::open(redis_host).unwrap();
    let pubsub = client.get_async_std_connection().await.unwrap().into_pubsub();

    // Spawn the subscriber_task to run asynchronously
    let _subscriber_task = async_std::task::spawn(subscribe_to_messages(pubsub, sender));
}
