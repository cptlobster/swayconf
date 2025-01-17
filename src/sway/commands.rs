/// Runtime command enumeration
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
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::sway::options::bindsym;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
pub enum Runtime {
    #[strum(to_string = "exec {command}")]
    Exec{ command: String },
    #[strum(to_string = "exec_always {command}")]
    ExecAlways{ command: String },
    #[strum(to_string = "bindsym {flags}{keys} {command}")]
    Bindsym{ flags: bindsym::BindArgs, keys: bindsym::BindKeys, command: Box<Runtime> },
    #[strum(to_string = "set ${name} {value}")]
    Set{ name: String, value: String },
}

#[cfg(test)]
mod tests {
    use crate::sway::options::bindsym;
    use super::*;
    
    #[test]
    fn test_to_sway() {
        let cmd1 = Runtime::Exec{command: "/bin/true".to_string()};
        let cmd2 = Runtime::Bindsym{
            flags: bindsym::BindArgs::default(),
            keys: bindsym::BindKeys::from(vec!["Mod4".to_string(), "X".to_string()]),
            command: Box::new(Runtime::Exec{command: "firefox".to_string()}),
        };
        let cmd3 = Runtime::Bindsym{
            flags: bindsym::BindArgs::from(vec![bindsym::Bindsym::ExcludeTitlebar]),
            keys: bindsym::BindKeys::from(vec!["Mod4".to_string(), "Shift".to_string()]),
            command: Box::new(Runtime::Exec{command: "ls -la ~".to_string()}),
        };
        let cmd4 = Runtime::Set{name: "foo".to_string(), value: "bar".to_string()};
        
        assert_eq!(cmd1.to_string(), "exec /bin/true");
        assert_eq!(cmd4.to_string(), "set $foo bar");
        assert_eq!(cmd2.to_string(), "bindsym Mod4+X exec firefox");
        assert_eq!(cmd3.to_string(), "bindsym --exclude-titlebar Mod4+Shift exec ls -la ~");
    }
    
    fn test_serde() {
        // println!("{}", toml::to_string(&cmd1).unwrap());
        // println!("{}", toml::to_string(&cmd2).unwrap());
        // println!("{}", toml::to_string(&cmd3).unwrap());
    }
}