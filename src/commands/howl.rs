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
    model::{channel::Message},
    client::{Context},
    framework::standard::{
        macros::command,
        CommandResult
    }
};

#[command]
async fn howl(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "MURGI CLAN AWOOOOOOOOOOOO").await?;

    Ok(())
}
