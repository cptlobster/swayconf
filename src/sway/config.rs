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
use std::fs::File;
use std::io::{Write, Result};
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::ptr::null;
use homedir::my_home;

/// A single configuration file
pub struct ConfigFile {
    path: PathBuf,
    commands: Vec<Commands>
}

impl ConfigFile {
    /// Create a new config file
    fn new(path: PathBuf, commands: Vec<Commands>) -> ConfigFile {
        ConfigFile{ path, commands }
    }

    /// Create a new config file at the default location (`~/.config/sway/config`)
    fn default(commands: Vec<Commands>) -> ConfigFile {
        let default_loc: PathBuf = my_home().unwrap().unwrap().join(".config/sway/config");
        ConfigFile::new(default_loc, commands)
    }

    /// Concatenate two config files (keeps the left file's name)
    fn concat(self, other: ConfigFile) -> ConfigFile {
        let mut new_cmds: Vec<Commands> = self.commands;
        let mut other_cmds: Vec<Commands> = other.commands;
        new_cmds.append(&mut other_cmds);
        ConfigFile::new(self.path, new_cmds)
    }

    fn write(self) -> Result<File> {
        File::open(self.path)
            .and_then(|mut f| {f.write(self.commands.to_string())})
    }
}

// this will let us convert the entire config struct into a single string representation
impl Display for ConfigFile {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.commands.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

/// A group of config files (ideally connected via include statements)
pub struct ConfigGroup {
    files: Vec<ConfigFile>
}

impl ConfigGroup {
    fn flatten(self) -> ConfigFile {
        self.files.iter().cloned().reduce(|left, right| left.concat(right)).unwrap()
    }

    fn write(self) -> Result<File> {
        let mut result: Result<File> = Err(NotFound.into());
        for file in self.files {
            match file.write() {
                Err(err) => { result = Err(err); break; }
                ok => result = ok,
            }
        }
        result
    }
}