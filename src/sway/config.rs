// <one line to give the program's name and a brief idea of what it does.>
// Copyright (C) 2024, 2025 Dustin Thomas <stdio@cptlobster.dev>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use crate::sway::options;
use crate::sway::options::{bind, exec, layout, ArgMap};
use crate::sway::runtime::Runtime;

/// Basic structure for a config file.
///
/// By default, Sway allows for configuration commands to be input in arbitrary order. While this
/// structure has a much more rigid arrangement than Sway normally allows, this provides much
/// simpler compatibility with Serde and allows for formatting your configs in TOML.
/// 
/// # Example
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
/// will render as the following Sway config (comments and blank lines are stripped):
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
    /// Default orientation and workspace layout
    #[serde(default)]
    default: Option<Defaults>,
    #[serde(default)]
    modes: Option<Modes>,
    /// User-defined bindsym commands
    #[serde(default)]
    bindsym: Option<HashMap<String, KeylessBindsym>>,
    /// User-defined bindcode commands
    #[serde(default)]
    bindcode: Option<HashMap<String, KeylessBindsym>>,
    #[serde(default)]
    bar: Option<Bar>,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Modes (HashMap<String, ModeCfg>);

impl Display for Modes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (k, v) in self.0.iter() {
            log::debug!("Converting mode {}...", k);
            let header = format!("# Configuration for mode {}", k);
            write!(f, "{}\nmode {} {{\n{}\n}}\n", header, k, indent(&v.to_string(), 4))?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModeCfg {
    // User defined bindsym commands for this mode
    bindsym: Option<HashMap<String, KeylessBindsym>>,
    // User defined bindcode commands for this mode
    bindcode: Option<HashMap<String, KeylessBindsym>>,
}

impl Display for ModeCfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let bindsym = stringify_bindsyms(&self.bindsym);
        let bindcode = stringify_bindcodes(&self.bindcode);
        write!(f, "{}{}", bindsym, bindcode)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    orientation: Option<options::DefaultOrientation>,
    layout: Option<layout::ConfigLayout>,
    border: Option<options::DefaultBorder>,
    floating_border: Option<options::DefaultBorder>,
}

impl Display for Defaults {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let layout = match self.layout {
            Some(ref w) => format!("workspace_layout {}\n", w),
            None => String::new()
        };
        let orientation = match self.orientation {
            Some(ref w) => format!("default_orientation {}\n", w),
            None => String::new()
        };
        let border = match self.border {
            Some(ref w) => format!("default_border {}\n", w),
            None => String::new()
        };
        let floating_border = match self.floating_border {
            Some(ref w) => format!("default_floating_border {}\n", w),
            None => String::new()
        };
        write!(f, "{}{}{}{}", layout, orientation, border, floating_border)
    }
}

/// Bindsym argument structure, minus the keys.
///
/// When assembling the config-level bindsym commands, the bind map will provide the keys / key
/// codes. This struct provides the rest of the arguments, as well as the runtime command to
/// execute.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KeylessBindsym {
    #[serde(default)]
    flags: ArgMap<bind::Bind>,
    #[serde(flatten)]
    command: Runtime
}

impl KeylessBindsym {
    pub fn new(flags: ArgMap<bind::Bind>, command: Runtime) -> Self {
        Self { flags, command }
    }
}

/// Arguments for generating swaybars. This may be refactored in a future update.
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

fn indent(content: &str, level: u8) -> String {
    let ind = (0..level).map(|_| " ").collect::<String>();
    content.lines().map(|s| format!("{}{}", ind, s)).collect::<Vec<String>>().join("\n")
}

fn with_comment_header(section: String, header: String) -> String {
    let comment = header.lines().map(|l| format!("# {l}")).collect::<Vec<String>>().join("\n");
    format!("{}\n{}\n\n", comment, section)
}

fn stringify_sets(sets: &Option<HashMap<String, String>>) -> String {
    log::debug!("Converting set commands...");
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

fn stringify_defaults (defaults: &Option<Defaults>) -> String {
    log::debug!("Converting default workspace settings...");
    match defaults {
        Some(d) => {
            let res0 = d.to_string();
            if res0.is_empty() {String::new()}
            else {
                with_comment_header(res0,
                    "Default workspace layout and orientation (using [defaults] table)".to_string()
                )
            }
        }
        None => String::new()
    }
}

fn stringify_bindsyms(bindsym: &Option<HashMap<String, KeylessBindsym>>) -> String {
    log::debug!("Converting bindsyms...");
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
    log::debug!("Converting bindcodes...");
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

fn stringify_exec(exec: &Option<Vec<exec::ExecParams>>) -> String {
    log::debug!("Converting startup applications (exec)...");
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

fn stringify_exec_always(exec_always: &Option<Vec<exec::ExecParams>>) -> String {
    log::debug!("Converting startup applications (exec_always)...");
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
    log::debug!("Converting bar commands...");
    match bar {
        Some(b) => with_comment_header(b.to_string(), "Swaybar configuration".to_string()),
        None => String::new()
    }
}

fn stringify_modes (modes: &Option<Modes>) -> String {
    log::debug!("Converting modes...");
    match modes {
        Some(m) => with_comment_header(m.to_string(), "Mode configuration".to_string()),
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
        write!(f, "{}{}{}{}{}{}{}{}{}",
               with_comment_header(String::new(), header.to_string()),
               stringify_sets(&self.set),
               stringify_exec(&self.exec),
               stringify_exec_always(&self.exec_always),
               stringify_defaults(&self.default),
               stringify_modes(&self.modes),
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
                exec::ExecParams::String("ls".to_string()),
                exec::ExecParams::String("/bin/bash".to_string()),
            ]
        );

        let mut keys = HashMap::new();
        keys.insert("Mod4+Shift".to_string(), KeylessBindsym::new(ArgMap::<bind::Bind>::default(), Runtime::Exec(exec::ExecParams::String("ls -la ~".to_string()))));
        keys.insert("Mod4+X".to_string(), KeylessBindsym::new(ArgMap::<bind::Bind>::default(), Runtime::Exec(exec::ExecParams::String("~/beans.sh".to_string()))));
        keys.insert("Mod4+Shift+Q".to_string(), KeylessBindsym::new(ArgMap::<bind::Bind>::default(), Runtime::Kill));
        
        config.bindsym = Some(keys);
        
        config.bar = Some(Bar{ id: "".to_string(), status_command: "i3blocks".to_string() });

        println!("{}", toml::to_string(&config).unwrap());
        println!("{}", &config.to_string());
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