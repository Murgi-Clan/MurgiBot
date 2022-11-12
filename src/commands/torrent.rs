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
use serde::{Serialize, Deserialize};
use serde_this_or_that::as_i64;
use chrono::prelude::Utc;
use chrono::DateTime;

// Struct to represent the structure of each torrent present
// in the Response
#[derive(Serialize, Deserialize, Debug)]
struct Torrent {
    url: String,
    title: String,
    #[serde(deserialize_with = "as_i64")]
    seed: i64,
    magnetlink: String,
    filesize: f64,
    category: String
}

// Structure of the Response gathered by the torrenting 
// service which contacts the Searx instance
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    query: String,
    number_of_results: f64,
    results: Vec<Torrent>
}

/// This function retrieves the message sent by the user, cleans it
/// with respect to the number of characters in the command, creates a query,
/// and then proceeds to retrieve the torrent data from the RSS Feed, and displays
/// it in the form of an embed message in Discord.
/// Retrieving and cleaning the message content
#[command]
async fn torrent(ctx: &Context, msg: &Message) -> CommandResult {
    let mut s = String::from(&msg.content);
    let search = s.split_off(10);
    let client = reqwest::Client::new();

    // Retrieving torrents from searx
    // Since searx works a bit differently, and I need to post data rather than get it,
    // the requests have to work a bit differently as well.
    let params = [("q", "!tpb ".to_string() + &search), ("format", "json".to_string())];
    let query: String = env::var("SEARX_INSTANCE").expect("Expected a Searx Instance Link") + "/search";
    let content = client.post(query)
        .form(&params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?
        .text()
        .await?;

    // Transmutes the data into JSON through serde
    let parsed: Response = serde_json::from_str(&content).unwrap();

    // TODO Filter by seeds
    // Takes the first 5 responses and pushes them into the Vector.
    let mut torrs = vec![];
    for trt in parsed.results.iter().take(5) {
        torrs.push((&trt.title, &trt.magnetlink, true));
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
