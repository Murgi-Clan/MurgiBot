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
use std::path::Path;

use config::{Config, ConfigError, Environment, File};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub discord_token: String,
    pub searx_instance: String
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        const CURRENT_DIR: &str = "config/default.toml";
        let s = Config::builder()
            .add_source(config::File::with_name(&CURRENT_DIR))
            .build()
            .unwrap();

        s.try_deserialize()
    }
}
