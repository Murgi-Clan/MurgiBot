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

use std::env;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use chrono::prelude::*;

use rss::{Channel, Item};

/// This function retrieves the message sent by the user, cleans it
/// with respect to the number of characters in the command, creates a query,
/// and then proceeds to retrieve the torrent data from the RSS Feed, and displays
/// it in the form of an embed message in Discord.
/// Retrieving and cleaning the message content
#[command]
async fn torrent(ctx: &Context, msg: &Message) -> CommandResult {
    let mut s = String::from(&msg.content);
    let search = s.split_off(10);

    // Creating the query to be sent to Jackett
    let query = env::var("JACKETT_RSS_FEED").expect("Expected an RSS Feed Link") + &search;
    let content = reqwest::get(&query).await?.bytes().await?;

    // Creates a vector of items, where each item has information of one torrent.
    let channel: Vec<Item> = Channel::read_from(&content[..])?.items().to_vec();

    // Takes the first 5 responses and pushes them into the Vector.
    let mut torrs = vec![];
    for trt in channel.iter().take(5) {
        torrs.push((trt.title().unwrap(), trt.link().unwrap(), true));
    }

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("The Murgi screeches.");
            m.embed(|e| {
                e.title("The Murgi Lords grace you.");
                e.description("Here's a list of torrents that has been bestowed upon thee.");
                // Need to convert this link into a string.
                e.url("https://www.youtube.com/watch?v=EGohSsaCJOU");
                // Probably can't put multiple links to stuff without it looking trash.
                e.fields(torrs);
                e.field(
                    "Those were some of the top results",
                    "If you didn't get what you want, then, maybe it's not here yet.",
                    false,
                );
                e.footer(|f| {
                    f.text("Jackett, Torrents, and Murgis.");
                    f
                });

                e.timestamp(DateTime::to_rfc3339(&Utc::now()));
                e
            });
            m
        })
        .await;

    // Error Handling for the developer
    if let Err(why) = msg {
        println!("The chicken faced an error: {:?}", why);
    }

    Ok(())
}
