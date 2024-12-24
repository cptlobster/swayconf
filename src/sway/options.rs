/// Enums for options for Sway commands
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
use strum::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Size {
    Shrink,
    Grow,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ContainerType {
    Container,
    Window,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Units {
    Px,
    Ppt,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Hierarchy {
    Parent,
    Child,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum FocusSibling {
    Prev,
    Next,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RelWorkspace {
    Prev,
    Next,
    Current,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Directional {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TogglableBool {
    Enable,
    Disable,
    Toggle,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
    Default,
    Stacking,
    Tabbed,
    SplitH,
    SplitV,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum LayoutCycle {
    Stacking,
    Tabbed,
    Split,
    SplitH,
    SplitV,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Split {
    Horizontal,
    Vertical,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Bindsym {
    WholeWindow,
    Border,
    ExcludeTitlebar,
    Release,
    Locked,
    ToCode,
    InputDevice(String),
    NoWarn,
    NoRepeat,
    Inhibited,
}