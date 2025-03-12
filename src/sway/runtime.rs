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

use serde::{Deserialize, Serialize};
use strum::Display;
use crate::sway::{options};
use crate::sway::criteria::CriteriaVec;
use crate::sway::options::{bind, exec, focus, layout, mov, resize, ArgMap};

/// Runtime commands for Sway.
///
/// Fun little piece of lore for writing your own configs: for enum variants that do not take
/// parameters, you have to represent them as "null" types for [serde] to not throw errors. The only
/// way to normally do this in TOML is like so:
/// ```toml
/// [bindsym]
/// "Mod4+Shift+q".kill = {}
/// ```
/// This would bind `Mod4+Shift+Q` to the `kill` command if `kill` was a variant of [Runtime]. We
/// get around this by representing parameterless commands in a separate enum ([ParamlessRuntime])
/// that are stored in an untagged variant of [Runtime].
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Runtime {
    #[strum(to_string = "allow_tearing {0}")]
    AllowTearing(bool),
    #[strum(to_string = "bindsym {flags}{keys} {command}")]
    BindCode{
        #[serde(default, flatten)]
        flags: ArgMap<bind::Bind>,
        keys: bind::BindCodes,
        #[serde(flatten)]
        command: Box<Runtime>
    },
    #[strum(to_string = "bindsym {flags}{keys} {command}")]
    BindSym {
        #[serde(default)]
        flags: ArgMap<bind::Bind>,
        keys: bind::BindKeys,
        #[serde(flatten)]
        command: Box<Runtime>
    },
    #[strum(to_string = "border {0}")]
    Border(options::Border),
    #[strum(to_string = "exec {0}")]
    Exec(exec::ExecParams),
    #[strum(to_string = "exec_always {0}")]
    ExecAlways(exec::ExecParams),
    Exit,
    #[strum(to_string = "floating {0}")]
    Floating(options::TogglableBool),
    #[strum(to_string = "focus {0}")]
    Focus(focus::FocusParams),
    #[strum(to_string = "{0} focus")]
    CriteriaFocus(CriteriaVec),
    #[strum(to_string = "for_window {criteria} {command}")]
    ForWindow {
        criteria: CriteriaVec,
        #[serde(flatten)]
        command: Box<Runtime>
    },
    Kill,
    #[strum(to_string = "layout {0}")]
    Layout(layout::LayoutParams),
    #[strum(to_string = "max_render_time {0}")]
    MaxRenderTime(options::MaxRenderTimeOpts),
    #[strum(to_string = "mode {0}")]
    Mode(String),
    #[strum(to_string = "move {0}")]
    Move(mov::MoveParams),
    Nop,
    Reload,
    #[strum(to_string = "rename workspace {0}")]
    Rename(options::RenameOpts),
    #[strum(to_string = "resize {0}")]
    Resize(resize::ResizeParams),
    #[strum(to_string = "scratchpad show")]
    Scratchpad,
    #[strum(to_string = "shortcuts_inhibitor {0}")]
    ShortcutsInhibitor(bool),
    #[strum(to_string = "split {0}")]
    Split(options::Split),
    #[strum(to_string = "set ${name} {value}")]
    Set{ 
        name: String,
        value: String
    },
    #[strum(to_string = "sticky {0}")]
    Sticky(options::TogglableBool),
    #[strum(to_string = "swap container with {0}")]
    Swap(options::Swap),
    #[strum(to_string = "title_format {0}")]
    TitleFormat(String),
    #[strum(to_string = "workspace {0}")]
    Workspace(options::Workspace),
}

#[cfg(test)]
mod tests {
    use crate::sway::options::bind;
    use crate::sway::options::exec::ExecParams;
    use super::*;
    
    #[test]
    fn test_to_sway() {
        let cmd1 = Runtime::Exec(ExecParams::String("/bin/true".to_string()));
        let cmd2 = Runtime::BindSym {
            flags: ArgMap::<bind::Bind>::default(),
            keys: bind::BindKeys::from(vec!["Mod4".to_string(), "X".to_string()]),
            command: Box::new(Runtime::Exec(ExecParams::String("firefox".to_string()))),
        };
        let mut am = ArgMap::<bind::Bind>::default();
        am.insert(bind::Bind::ExcludeTitlebar, true);
        let cmd3 = Runtime::BindSym {
            flags: am,
            keys: bind::BindKeys::from(vec!["Mod4".to_string(), "Shift".to_string()]),
            command: Box::new(Runtime::Exec(ExecParams::String("ls -la ~".to_string()))),
        };
        let cmd4 = Runtime::Set{name: "foo".to_string(), value: "bar".to_string()};
        
        assert_eq!(cmd1.to_string(), "exec /bin/true");
        assert_eq!(cmd4.to_string(), "set $foo bar");
        assert_eq!(cmd2.to_string(), "bindsym Mod4+X exec firefox");
        assert_eq!(cmd3.to_string(), "bindsym --exclude-titlebar Mod4+Shift exec ls -la ~");
    }
}