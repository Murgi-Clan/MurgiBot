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

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Amurgi, my brethren.");
            m.embed(|e| {
                e.title("The Murgi Lord graces you with assistance.");
                e.description("If you're clueless about how to work with the Murgi Bot, hit up `@Hel`, he's being all useless anyway. Also, my prefix is `m>`, please keep that in mind.");
                e.url("https://youtu.be/AIXkJecFxeA");
                e.fields(vec![
                         ("m>help", "A simple message on how to work with the Murgi Bot.", true),
                         ("m>info", "About the Murgi Bot.", true),
                ]);
                e.fields(vec![
                         ("m>howl", "Exhibits it's patriotism towards the Murgi Clan.", true),
                ]);
                e.fields(vec![
                         ("m>torrent", "Instructs Murgi Bot to go find some torrents.", true),
                ]);
                e.fields(vec![
                         // Might need to implement random numbers without arguments and with one argument.
                         ("m>random", "Rolls for a random number between two given numbers.", true), 
                         ("m>d4", "Rolls a d4 dice", true),
                         ("m>d6", "Rolls a d6 dice", true),
                         ("m>d8", "Rolls a d8 dice", true),
                         ("m>d10", "Rolls a d10 dice", true),
                         ("m>d12", "Rolls a d12 dice", true),
                         ("m>d20", "Rolls a d20 dice", true),
                ]);
                e.field("More features are coming soon!", "If you didn't get what you want, then, `@Hel` probably hasn't worked on it yet, that guy's kinda lazy, no cap.", false);
                e.footer(|f| {
                    f.text("MURGI Ltd.");
                    f
                });

                e.timestamp(chrono::Utc::now());
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

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("About Murgi Bot.");
            m.embed(|e| {
                e.title("The Phoenix of the Murgi Clan.");
                e.description("The creator of the Murgi Bot, is `@dat-adi` or `@DatScum` of the Murgi Clan. The tools that were used for the operational Murgi Bot are as listed below.");
                e.url("https://github.com/dat-adi/MurgiBot");
                e.fields(vec![
                         ("Discord API", "Underlying API for the functioning of the bot.", true),
                         ("Rust", "The fastest language in the current day, kinda overkill for a Discord Bot.", true),
                         ("Serenity", "Library for interacting with the Discord API through Rust.", true),
                         // ("Songbird", "Cross library for voice system in Discord.", true),
                         ("Jackett", "Self hosted Docker Container for querying torrents.", true),
                         ("GitHub", "Code hosting platform, and workflow management.", true),
                         ("Docker", "Containerizing application for a isolated environment.", true),
                ]);
                e.field("Open Source!", "This project is completely FOSS, and was built just to entertain my gang of friends, read the LICENSE for more info!", false);
                e.footer(|f| {
                    f.text("@dat-adi, MURGI Ltd.");
                    f
                });

                e.timestamp(chrono::Utc::now());
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
