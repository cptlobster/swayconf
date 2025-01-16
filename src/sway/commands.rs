/// Configuration command generation and parsing
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
use subenum::subenum;
use crate::sway::options;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

/// All top-level command declarations. These are developed using the criteria specified in the `sway(5)` manpage.
#[subenum(Config, Runtime)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    #[subenum(Config)]
    Bar{ bar_id: String, subcommands: String },
    #[subenum(Runtime)]
    Exit,
    #[subenum(Runtime)]
    Floating(options::TogglableBool),
    #[subenum(Runtime)]
    Focus(SubFocus),
    #[subenum(Runtime)]
    Layout(SubLayout),
    #[subenum(Runtime)]
    Move(SubMove),
    #[subenum(Runtime)]
    Reload,
    #[subenum(Runtime)]
    Resize{ change: options::Size, x: Option<u8>, y: Option<u8>, unit: options::Units },
    #[subenum(Runtime)]
    Split(options::Split),
    #[subenum(Config, Runtime)]
    Bindsym{ flags: Vec<options::Bindsym>, keys: Vec<String>, command: Box<Runtime> },
    #[subenum(Config, Runtime)]
    Exec(String),
    #[subenum(Config, Runtime)]
    ExecAlways(String),
    #[subenum(Config, Runtime)]
    Kill,
    #[subenum(Config, Runtime)]
    Set{name: String, value: String},
    #[subenum(Runtime)]
    Workspace{ number: u8, name: Option<String> },
    #[subenum(Config, Runtime)]
    Else(String),
    #[subenum(Config)]
    Comment(String),
    #[subenum(Config)]
    Blank,
    #[subenum(Config)]
    Include(PathBuf),
}

/// Subcommands for focus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubFocus {
    Directional(options::Directional),
    Sibling(options::FocusSibling),
    Hierarchy(options::Hierarchy),
    OutputDirectional(options::Directional),
    OutputNamed(String),
}

/// Subcommands for layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubLayout {
    Set(options::Layout),
    Cycle(options::LayoutCycleSingle),
    CycleList(Vec<options::LayoutCycleMulti>),
}

/// Subcommands for move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubMove {
    Directional{direction: options::Directional, px: Option<u8>},
    Coordinates{x: i8, y: i8, x_unit: options::Units, y_unit: options::Units, absolute: bool},
    Center{absolute: bool},
    ToCursor,
    ToWorkspace(options::RelWorkspace),
    ToWorkspaceOnOutput(options::FocusSibling),
    BackAndForth,
    ToDirectionalOutput(options::Directional),
    ToNamedOutput(String),
}

// implement Display so that we can just use format! and to_string() to convert commands to strings
impl Display for Commands {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Commands::Bar{bar_id, subcommands} => { write!(f, "bar {} {{\n{}\n}})", bar_id, subcommands) }
            Commands::Bindsym{flags, keys, command} => {
                match *command.clone() {
                    Runtime::Bindsym{flags: _, keys: _, command: _} => panic!("Nested bindsyms are not allowed"),
                    c => {
                        let key_str = keys.join("+");
                        if flags.is_empty() { write!(f, "bindsym {} {}", key_str, c) }
                        else {
                            let flag_str = flags.iter().map(|bsf| bsf.to_string()).collect::<Vec<String>>().join(" ");
                            write!(f, "bindsym {} {} {}", flag_str, key_str, c)
                        }
                    }
                }
            }
            Commands::Blank => { write!(f, "") },
            Commands::Comment(content) => { write!(f, "# {}", content) }
            Commands::Else(command) => { write!(f, "{}", command) }
            Commands::Exec(command) => { write!(f, "exec {}", command) }
            Commands::ExecAlways(command) => { write!(f, "exec_always {}", command) }
            Commands::Exit => { write!(f, "exit") }
            Commands::Focus(focus) => { write!(f, "focus {}", focus) }
            Commands::Floating(val) => { write!(f, "floating {}", val) }
            Commands::Include(path) => { write!(f, "include {}", path.display()) }
            Commands::Kill => { write!(f, "kill") }
            Commands::Layout(layout) => { write!(f, "layout {}", layout) }
            Commands::Move(movement) => { write!(f, "move {}", movement) }
            Commands::Reload => { write!(f, "reload") }
            Commands::Resize{change, x, y, unit} => {
                if x.is_some() && y.is_none() {
                    write!(f, "resize {} width {} {}", change, x.unwrap(), unit)
                } else if y.is_some() && x.is_none() {
                    write!(f, "resize {} height {} {}", change, y.unwrap(), unit)
                } else {
                    panic!("Only one of x or y must be specified")
                }
            }
            Commands::Set{name, value} => { write!(f, "set ${} {}", name, value) }
            Commands::Split(split) => { write!(f, "split {}", split) }
            Commands::Workspace{number, name} => {
                match name {
                    Some(name_str) => write!(f, "workspace {} {}", number, name_str),
                    None => write!(f, "workspace {}", number)
                }
            }
        }
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        <Runtime as Into<Commands>>::into(self.clone()).fmt(f)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        <Config as Into<Commands>>::into(self.clone()).fmt(f)
    }
}


impl Display for SubFocus {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SubFocus::Directional(dir) => { write!(f, "{}", dir) }
            SubFocus::Hierarchy(dir) => { write!(f, "{}", dir) }
            SubFocus::Sibling(dir) => { write!(f, "{} sibling", dir) }
            SubFocus::OutputDirectional(dir) => { write!(f, "output {}", dir) }
            SubFocus::OutputNamed(name) => { write!(f, "output {}", name) }
        }
    }
}

impl Display for SubLayout {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SubLayout::Set(layout) => { write!(f, "{}", layout) }
            SubLayout::Cycle(layout) => { write!(f, "toggle {}", layout)}
            SubLayout::CycleList(layouts) => { write!(f, "toggle {}", layouts.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ")) }
        }
    }
}

impl Display for SubMove {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SubMove::Directional{direction, px} => {
                match px {
                    Some(mag) => write!(f, "{} {} px", direction, mag),
                    None => write!(f, "{}", direction)
                }
            }
            SubMove::Coordinates{x, x_unit, y, y_unit, absolute} => {
                if *absolute {
                    write!(f, "absolute position {} {} {} {}", x, x_unit, y, y_unit)
                }
                else {
                    write!(f, "position {} {} {} {}", x, x_unit, y, y_unit)
                }
            }
            SubMove::Center{absolute} => {
                if *absolute {
                    write!(f, "absolute position center")
                }
                else {
                    write!(f, "position center")
                }
            }
            SubMove::ToCursor => {
                write!(f, "position cursor")
            }
            SubMove::ToWorkspace(ws) => write!(f, "container to workspace {}", ws),
            SubMove::ToWorkspaceOnOutput(ws) => write!(f, "container to workspace {}_on_output", ws),
            SubMove::BackAndForth => write!(f, "container to workspace back_and_forth"),
            SubMove::ToDirectionalOutput(dir) => write!(f, "container to output {}", dir),
            SubMove::ToNamedOutput(name) => write!(f, "container to output {}", name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        // TODO: setup some more testing, make it parameter based?
        // swayconf struct representation of configuration strings
        let command1 = Runtime::Exec("/bin/bash".to_string());
        let command2 = Runtime::Layout(SubLayout::Set(options::Layout::Tabbed));
        let command3 = Config::Bindsym{flags: vec![], keys: vec!["Mod4".to_string(), "a".to_string()], command: Box::new(command1.clone())};
        let comment = Config::Comment("this is a test comment".to_string());

        // the configuration strings as they would appear in swayconf
        let res1 = "exec /bin/bash".to_string();
        let res2 = "layout tabbed".to_string();
        let res3 = format!("bindsym Mod4+a {}", res1);
        let res_comment = "# this is a test comment".to_string();

        assert_eq!(command1.to_string(), res1);
        assert_eq!(command2.to_string(), res2);
        assert_eq!(command3.to_string(), res3);
        assert_eq!(comment.to_string(), res_comment);
    }

}