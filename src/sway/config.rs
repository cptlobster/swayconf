/// Struct for generating full config files
//     Copyright (C) 2024  Dustin Thomas <io@cptlobster.dev>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.z

use crate::sway::commands::Commands;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Config {
    commands: Vec<Commands>
}

impl Config {
    fn new(commands: Vec<Commands>) -> Config {
        Config{ commands }
    }

    fn concat(self, other: Config) -> Config {
        let mut new_cmds: Vec<Commands> = self.commands;
        let mut other_cmds: Vec<Commands> = other.commands;
        new_cmds.append(&mut other_cmds);
        Config::new(new_cmds)
    }
}

// this will let us convert the entire config struct into a single string representation
impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.commands.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("\n"))
    }
}