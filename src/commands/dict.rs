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

#[command]
async fn dict(ctx: &Context, msg: &Message) -> CommandResult {
    let mut search_word = String::from(&msg.content);
    search_word = search_word.split_off(7);

    let api_key = env::var("MERRIAM_API_KEY").expect("Expected an API key");
    let query: String = format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", &search_word, &api_key);
    println!("{:#?}", &query);

    Ok(())
}
