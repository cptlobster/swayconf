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
use crate::sway::options::{bind, exec, ArgList};
use crate::sway::options::exec::ExecParams;
use crate::sway::runtime::Runtime;

/// Basic structure for a config file.
///
/// By default, Sway allows for configuration commands to be input in arbitrary order. While this
/// structure has a much more rigid arrangement than Sway normally allows, this provides much
/// simpler compatibility with Serde and allows for formatting your configs in TOML.
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
    exec: Option<Vec<exec::ExecParams>>,
    /// Startup commands (exec-always)
    /// 
    /// These commands will run when Sway is launched and when reload is called
    #[serde(default)]
    exec_always: Option<Vec<exec::ExecParams>>,
    /// User-defined bindsym commands
    #[serde(default)]
    bindsym: Option<HashMap<String, KeylessBindsym>>,
    /// User-defined bindcode commands
    #[serde(default)]
    bindcode: Option<HashMap<String, KeylessBindsym>>,
    #[serde(default)]
    bar: Option<Bar>,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KeylessBindsym {
    #[serde(default)]
    flags: ArgList<bind::Bind>,
    #[serde(flatten)]
    command: Runtime
}

impl KeylessBindsym {
    pub fn new(flags: ArgList<bind::Bind>, command: Runtime) -> Self {
        Self { flags, command }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Bar {
    #[serde(default)]
    id: String,
    status_command: String
}

impl Display for Bar {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.id.as_str() {
            "" => write!(f, "bar {{\n    status_command {}\n}}", self.status_command),
            _ => write!(f, "bar {} {{\n    status_command {}\n}}", self.id, self.status_command)
        }
    }
}

impl Bar {
    fn new(id: String, status_command: String) -> Self {
        Bar{ id, status_command }
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

fn stringify_bindcodes(bindcode: &Option<HashMap<String, KeylessBindsym>>) -> String {
    match bindcode {
        Some(s) => {
            if s.is_empty() {String::new()}
            else {
                with_comment_header(
                    s.iter().map(|(k, KeylessBindsym{flags, command})|
                        format!("bindcode {flags}{k} {command}")
                    ).collect::<Vec<String>>().join("\n"),
                    "User-defined bindcode commands (using [bindcode] table)".to_string()
                )
            }
        }
        None => String::new()
    }
}

fn stringify_exec(exec: &Option<Vec<ExecParams>>) -> String {
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

fn stringify_exec_always(exec_always: &Option<Vec<ExecParams>>) -> String {
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

fn stringify_bar (bar: &Option<Bar>) -> String {
    match bar {
        Some(b) => with_comment_header(b.to_string(), "Swaybar configuration".to_string()),
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
        write!(f, "{}{}{}{}{}{}{}",
               with_comment_header(String::new(), header.to_string()),
               stringify_sets(&self.set),
               stringify_exec(&self.exec),
               stringify_exec_always(&self.exec_always),
               stringify_bindsyms(&self.bindsym),
               stringify_bindcodes(&self.bindcode),
               stringify_bar(&self.bar)
        )
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
                ExecParams::String("ls".to_string()),
                ExecParams::String("/bin/bash".to_string()),
            ]
        );

        let mut keys = HashMap::new();
        keys.insert("Mod4+Shift".to_string(), KeylessBindsym::new(ArgList::<bind::Bind>::default(), Runtime::Exec(ExecParams::String("ls -la ~".to_string()))));
        keys.insert("Mod4+X".to_string(), KeylessBindsym::new(ArgList::<bind::Bind>::default(), Runtime::Exec(ExecParams::String("~/beans.sh".to_string()))));

        config.bindsym = Some(keys);
        
        config.bar = Some(Bar{ id: "".to_string(), status_command: "i3blocks".to_string() });

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
            \n\"$mod+Space\".floating = \"toggle\"\
            \n\"$mod+Shift+Space\".floating = false\
            \n\"$mod+X\".exec.command = \"~/beans.sh\""
        ).unwrap();

        println!("{}", cfg.to_string());
    }
}