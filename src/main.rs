/*
 * Murgi Bot, a Discord Bot made for the Murgi Clan on Discord.
 * Copyright (C) 2021  G V Datta Adithya
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 */

mod commands;

use commands::{dice::*, help::*, howl::*, torrent::*};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{macros::group, StandardFramework},
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
};

use regex::Regex;
use std::env;

struct Handler;

// Defining a howl counter, so, when it hits 4 counts, it howls.
static mut HOWL_COUNTER: i32 = 0;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        unsafe {
            let howl_checker = Regex::new(r"^MURGI CLAN AW[O]*$").unwrap();
            if howl_checker.is_match(&msg.content) {
                HOWL_COUNTER += 1;
                if HOWL_COUNTER == 4 {
                    let response = MessageBuilder::new()
                        .push("MURGI CLAN AWOOOOOOOOOOOOOOOOOO")
                        .build();

                    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }

                    HOWL_COUNTER = 0;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(howl, torrent, roll, random, d4, d6, d8, d10, d12, d20, hello, help, info)]
struct General;

#[tokio::main]
async fn main() {
    // Loading the environment variables
    // Initializing the logger to use environment variables
    // This is a bit intense, so, proceed with caution.
    // tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("m>")) // set the bot's prefix to "m>"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
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
