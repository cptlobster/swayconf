/// Config-exclusive configuration management
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

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use crate::sway::options::bind;
use crate::sway::commands::Runtime;

/// Basic structure for a config file. While this structure is more defined than the Sway config
/// file normally allows, this provides simple compatibility with Serde and allows for formatting
/// your configs in TOML.
/// 
/// ## Example
/// This TOML config:
/// ```toml
/// exec = ["ls", "/bin/bash"]
/// 
/// [set]
/// mod = "Mod4"
/// 
/// [bindsym]
/// "$mod+Shift".exec.command = "ls -la"
/// "$mod+X".exec.command = "~/beans.sh"
/// ```
/// will render as the following Sway config (there will be slightly more comments):
/// ```text
/// set $mod Mod4
/// exec ls
/// exec /bin/bash
/// bindsym $mod+Shift exec ls -la
/// bindsym $mod+X exec ~/beans.sh
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Set Sway config variables.
    #[serde(default)]
    set: Option<HashMap<String, String>>,
    /// Startup commands (exec)
    /// 
    /// Note that these will only be run once when Sway is launched; NOT when reload is called
    /// Use exec-always if you need this command run on reload
    #[serde(default)]
    exec: Option<Vec<String>>,
    /// Startup commands (exec-always)
    /// 
    /// These commands will run when Sway is launched and when reload is called
    #[serde(default)]
    exec_always: Option<Vec<String>>,
    /// User-defined bindsym commands
    #[serde(default)]
    bindsym: Option<HashMap<String, KeylessBindsym>>,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KeylessBindsym {
    #[serde(default)]
    flags: bind::BindFlags,
    #[serde(flatten)]
    command: Runtime
}

impl KeylessBindsym {
    pub fn new(flags: bind::BindFlags, command: Runtime) -> Self {
        Self { flags, command }
    }
}

fn with_comment_header(section: String, header: String) -> String {
    let comment = header.lines().map(|l| format!("# {l}")).collect::<Vec<String>>().join("\n");
    format!("{}\n{}\n\n", comment, section)
}

fn stringify_sets(sets: &Option<HashMap<String, String>>) -> String {
    match sets {
        Some(s) => {
            if s.is_empty() {String::new()}
            else {
                with_comment_header(s.iter().map(|(k, v)|
                    format!("set ${k} {v}")
                ).collect::<Vec<String>>().join("\n"),
                "Variables (using [set] table)".to_string())
            }
        }
        None => String::new()
    }
}

fn stringify_bindsyms(bindsym: &Option<HashMap<String, KeylessBindsym>>) -> String {
    match bindsym {
        Some(s) => {
            if s.is_empty() {String::new()}
            else {
                with_comment_header(
                    s.iter().map(|(k, KeylessBindsym{flags, command})|
                        format!("bindsym {flags}{k} {command}")
                    ).collect::<Vec<String>>().join("\n"),
                    "User-defined bindsym commands (using [bindsym] table)".to_string()
                )
            }
        }
        None => String::new()
    }
}

fn stringify_exec(exec: &Option<Vec<String>>) -> String {
    match exec {
        Some(s) => {
            if s.is_empty() {String::new()}
            else {
                with_comment_header(
                    s.iter().map(|s| format!("exec {s}")).collect::<Vec<String>>().join("\n"),
                    "Startup commands (using exec array)\
                    \nNote: these will only be run once; NOT when reload is called\
                    \nUse exec-always if you need this command run on reload".to_string()
                )
            }
        }
        None => String::new()
    }
}

fn stringify_exec_always(exec_always: &Option<Vec<String>>) -> String {
    match exec_always {
        Some(s) => {
            if s.is_empty() {String::new()}
            else {
                with_comment_header(
                    s.iter().map(|s| format!("exec-always {s}")).collect::<Vec<String>>().join("\n"),
                    "Startup commands (using exec-always array)\
                    \nNote: these will be run every time that reload is called".to_string())
            }
        }
        None => String::new()
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let header =
            "This configuration was generated by the swayconf configurator.\
            \nPlease note that this program does NOT validate your configuration, you\
            \nwill need to run `sway -c [config file] -C` to do so.\
            \n\
            \nFor more information, please visit https://github.com/cptlobster/swayconf.";
        write!(f, "{}{}{}{}{}",
               with_comment_header(String::new(), header.to_string()),
               stringify_sets(&self.set),
               stringify_exec(&self.exec),
               stringify_exec_always(&self.exec_always),
               stringify_bindsyms(&self.bindsym))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sway() {
        let mut config = Config::default();
        config.exec = Some(
            vec![
                "ls".to_string(),
                "/bin/bash".to_string(),
            ]
        );

        let mut keys = HashMap::new();
        keys.insert("Mod4+Shift".to_string(), KeylessBindsym::new(bind::BindFlags::default(), Runtime::Exec { command: "ls -la ~".to_string() }));
        keys.insert("Mod4+X".to_string(), KeylessBindsym::new(bind::BindFlags::default(), Runtime::Exec { command: "~/beans.sh".to_string() }));

        config.bindsym = Some(keys);

        println!("{}", config.to_string());
    }

    #[test]
    fn test_serde() {
        let cfg: Config = toml::from_str(
            "exec = [\"ls\", \"/bin/bash\"]\
            \n\
            \n[set]\
            \nmod = \"Mod4\"\
            \n[bindsym]\
            \n\"$mod+Shift\".exec.command = \"ls -la\"\
            \n\"$mod+X\".exec.command = \"~/beans.sh\""
        ).unwrap();

        println!("{}", cfg.to_string());
    }
}