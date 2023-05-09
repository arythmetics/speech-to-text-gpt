use futures::StreamExt;
use redis::{AsyncCommands, aio::Connection, aio::PubSub};

async fn publish_messages(mut con: Connection, message: String) {
    let channel_name = "my_channel";

    // Publish the message to the channel
    let _: () = con.publish(channel_name, &message).await.unwrap();
    println!("Published message '{}' to channel '{}'", message, channel_name);
}

async fn subscribe_to_messages(mut pubsub: PubSub) {
    let channel_name = "my_channel";

    // Subscribe to the channel
    pubsub.subscribe(channel_name).await.unwrap();

    // Listen for messages
    while let Some(msg) = pubsub.on_message().next().await {
        let payload: String = msg.get_payload().unwrap();
        println!("Received message: {}", payload);
    }
}

pub async fn run() {
    let client = redis::Client::open("redis://localhost:6379/").unwrap();
    let pubsub = client.get_async_std_connection().await.unwrap().into_pubsub();

    // Spawn the subscriber_task to run asynchronously
    let subscriber_task = async_std::task::spawn(subscribe_to_messages(pubsub));

    // Wait for the subscriber_task to complete (it won't, since it runs in an infinite loop)
    let _ = subscriber_task.await;
}
