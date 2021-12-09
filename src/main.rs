mod commands;

use commands::{howl::*};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "m>howl" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" summoned the Murgi overlord in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(howl)]
struct General;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("m>")) // set the bot's prefix to "m>"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
