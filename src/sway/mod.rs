mod commands;
mod options;

use commands::Commands;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Struct for holding a single config file.
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