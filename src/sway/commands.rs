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
use strum::{EnumString, Display};
use crate::sway::options;
#[subenum(Config, Runtime)]
#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Commands {
    #[subenum(Config)]
    Bar{bar_id: String, subcommands: String},
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
    Resize{change: options::Size, x: Option<i8>, y: Option<i8>, x_unit: options::Units, y_unit: options::Units},
    #[subenum(Runtime)]
    Split(options::Split),
    #[subenum(Config, Runtime)]
    Bindsym{flags: options::Bindsym, keys: Vec<String>, command: Box<Runtime>},
    #[subenum(Config, Runtime)]
    Exec(String),
    #[subenum(Config, Runtime)]
    ExecAlways(String),
    #[subenum(Config, Runtime)]
    Kill,
    #[subenum(Config, Runtime)]
    Set{name: String, value: String},
    #[subenum(Config, Runtime)]
    Workspace{number: i8, name: Option<String>},
    #[subenum(Config, Runtime)]
    Else(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SubFocus {
    Direction(options::Directional),
    Sibling(options::FocusSibling),
    Hierarchy(options::Hierarchy),
    OutputDirection(options::Directional),
    OutputNamed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SubLayout {
    Set(options::Layout),
    Cycle(options::LayoutCycle),
    CycleList(Vec<options::LayoutCycle>),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
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