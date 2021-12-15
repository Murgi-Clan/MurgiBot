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

use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use chrono::prelude::*;

fn get_rand(u: &i32, v: &i32) -> i32 {
    // Simple function to get the random number
    // created as the Thread interfered with the macro
    let mut rng = rand::thread_rng();

    // Returning the randomized result
    rng.gen_range(*u..*v)
}

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    // Add parameters
    // Below given is probably an extremely inefficient
    // solution to get integer arguments from a string, please forgive me.
    let mut s = String::from(&msg.content);
    s = s.split_off(9);
    let args = s.split_whitespace().collect::<Vec<_>>();

    let val0 = args[0].parse::<i32>().unwrap();
    let val1 = args[1].parse::<i32>().unwrap();

    let res = format!("{}", get_rand(&val0, &val1));

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Respect to the Murgi.");
            m.embed(|e| {
                e.title("So, you've decided to take things into your hands.");
                e.description("As a Murgi takes up the mantle, the battlefield excites.");
                e.field("All that you have is a dice in your pocket.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d4(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &4));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d4.");
                e.description("So, now, you face the consequences of this roll.");
                e.field(
                    "A dice falls from the sky, as you pray to the Murgi Lords.",
                    res,
                    false,
                );
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d6(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &6));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d6.");
                e.description("You hear screeches, and oaths of glory.");
                e.field("A dice makes it's way across the battlefield, as you struggle to open your eyes.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d8(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &8));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d8.");
                e.description("Stars quake the earth, as they strike down upon humanity.");
                e.field(
                    "What seemed to be a shooting star, was in fact a dice.",
                    res,
                    false,
                );
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d10(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &10));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d10.");
                e.description("The world scars me, as I lose my feathers.");
                e.field(
                    "A feather blown away by the wind, reveals underneath a dice.",
                    res,
                    false,
                );
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d12(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &12));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d12.");
                e.description("Phoenixes fall, as the world seems to turn inward.");
                e.field("The light vanishes, but, you manage to find an object on the ground. It's a dice.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn d20(ctx: &Context, msg: &Message) -> CommandResult {
    let res = format!("{}", get_rand(&1, &20));

    // Creates an embed message
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("You hear some clucks nearby.");
            m.embed(|e| {
                e.title("Somewhere, somehow, a chicken rolled a d20.");
                e.description("Embers resonate, as the final phoenix rests.");
                e.field("Four chickens rise to the weight of the world, each holding a dice of their own.", res, false);
                e.footer(|f| {
                    f.text("Dice Rollers, Randoms and Murgis.");
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
