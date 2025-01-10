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

use crate::sway::commands::Config;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Write, Result};
use std::path::PathBuf;
use homedir::my_home;

/// A single configuration file
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigFile {
    path: PathBuf,
    commands: Vec<Config>
}

impl ConfigFile {
    /// Create a new config file
    fn new(path: PathBuf, commands: Vec<Config>) -> ConfigFile {
        ConfigFile{ path, commands }
    }

    /// Create a new config file at the default location (`~/.config/sway/config`)
    fn default(commands: Vec<Config>) -> ConfigFile {
        let default_loc: PathBuf = my_home().unwrap().unwrap().join(".config/sway/config");
        ConfigFile::new(default_loc, commands)
    }

    /// Concatenate two config files (keeps the left file's name)
    fn concat(self, other: ConfigFile) -> ConfigFile {
        let mut new_cmds: Vec<Config> = self.commands;
        let mut other_cmds: Vec<Config> = other.commands;
        new_cmds.append(&mut other_cmds);
        ConfigFile::new(self.path, new_cmds)
    }

    fn write(self) -> Result<usize> {
        File::open(self.path.clone())
            .and_then(|mut f| {f.write(self.to_string().as_bytes())})
    }
}

// this will let us convert the entire config struct into a single string representation
impl Display for ConfigFile {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.commands.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

/// A group of config files (ideally connected via include statements)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigGroup {
    files: Vec<ConfigFile>
}

impl ConfigGroup {
    fn flatten(self) -> ConfigFile {
        self.files.iter().cloned().reduce(|left, right| left.concat(right)).unwrap()
    }

    fn write(self) -> Result<usize> {
        let mut result: Result<usize> = Ok(0);
        for file in self.files {
            match file.write() {
                Err(err) => { result = Err(err); break; }
                Ok(s) => result = Ok(result? + s),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::sway::commands::{Config, Runtime, SubMove};
    use crate::sway::options::Directional;
    use super::*;

    #[test]
    fn test_config_to_string() {
        let commands: Vec<Config> = vec![
            Config::ExecAlways("/bin/echo \"this is a command!\"".to_string()),
            Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "Space".to_string()], command: Box::new(Runtime::Exec("/bin/bash".to_string()))},
            Config::Comment("move the currently focused window around".to_string()),
            Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "W".to_string()], command: Box::new(Runtime::Move(SubMove::Directional{direction: Directional::Up, px: None}))},
            Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "S".to_string()], command: Box::new(Runtime::Move(SubMove::Directional{direction: Directional::Down, px: None}))},
            Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "A".to_string()], command: Box::new(Runtime::Move(SubMove::Directional{direction: Directional::Left, px: None}))},
            Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "D".to_string()], command: Box::new(Runtime::Move(SubMove::Directional{direction: Directional::Right, px: None}))},
        ];

        let expected: String = "exec_always /bin/echo \"this is a command!\"\n\
        bindsym Mod4+Space exec /bin/bash\n\
        # move the currently focused window around\n\
        bindsym Mod4+W move up\n\
        bindsym Mod4+S move down\n\
        bindsym Mod4+A move left\n\
        bindsym Mod4+D move right".to_string();

        let file = ConfigFile::default(commands);
        assert_eq!(expected, file.to_string());
    }
}