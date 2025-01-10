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

use std::fmt::{Display, Formatter, Result as FmtResult};
use subenum::subenum;
use crate::sway::options::ContainerType::Container;
use crate::sway::options::Size::Grow;

/// Possible options for resize commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Shrink,
    Grow,
}

/// Possible options for container types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerType {
    Container,
    Window,
}

/// Possible options for units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Units {
    Px,
    Ppt,
}

/// Possible options for parent/child hierarchy commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hierarchy {
    Parent,
    Child,
}

/// Possible options for sibling hierarchy commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusSibling {
    Prev,
    Next,
}

/// Possible options for relative workspaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelWorkspace {
    Prev,
    Next,
    Current,
}

/// Possible directional arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directional {
    Up,
    Down,
    Left,
    Right,
}

/// "Togglable boolean"; Has true/false value, but also "toggle" which will switch from true to false and vice versa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TogglableBool {
    Enable,
    Disable,
    Toggle,
}

/// Possible layout options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Default,
    Stacking,
    Tabbed,
    SplitH,
    SplitV,
}

/// Possibly options for cycling layouts.
#[subenum(LayoutCycleSingle, LayoutCycleMulti)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutCycle {
    #[subenum(LayoutCycleMulti)]
    Stacking,
    #[subenum(LayoutCycleMulti)]
    Tabbed,
    #[subenum(LayoutCycleSingle, LayoutCycleMulti)]
    Split,
    #[subenum(LayoutCycleMulti)]
    SplitH,
    #[subenum(LayoutCycleMulti)]
    SplitV,
    #[subenum(LayoutCycleSingle)]
    All,
}

/// Possible split options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Split {
    Horizontal,
    Vertical,
    None,
}

/// Possible flags for bindsym commands.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Grow => write!(f, "grow"),
            Size::Shrink => write!(f, "shrink"),
        }
    }
}

impl Display for ContainerType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ContainerType::Window => write!(f, "window"),
            Container => write!(f, "container"),
        }
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Units::Px => write!(f, "px"),
            Units::Ppt => write!(f, "ppt"),
        }
    }
}

impl Display for Hierarchy {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Hierarchy::Parent => write!(f, "parent"),
            Hierarchy::Child => write!(f, "child"),
        }
    }
}

impl Display for FocusSibling {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            FocusSibling::Prev => write!(f, "prev"),
            FocusSibling::Next => write!(f, "next"),
        }
    }
}

impl Display for RelWorkspace {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            RelWorkspace::Prev => write!(f, "prev"),
            RelWorkspace::Next => write!(f, "next"),
            RelWorkspace::Current => write!(f, "current"),
        }
    }
}

impl Display for Directional {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Directional::Up => write!(f, "up"),
            Directional::Down => write!(f, "down"),
            Directional::Left => write!(f, "left"),
            Directional::Right => write!(f, "right"),
        }
    }
}

impl Display for TogglableBool {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            TogglableBool::Toggle => write!(f, "toggle"),
            TogglableBool::Enable => write!(f, "enable"),
            TogglableBool::Disable => write!(f, "disable"),
        }
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Layout::Default => write!(f, "default"),
            Layout::Stacking => write!(f, "stacking"),
            Layout::Tabbed => write!(f, "tabbed"),
            Layout::SplitH => write!(f, "splith"),
            Layout::SplitV => write!(f, "splitv"),
        }
    }
}
impl Display for LayoutCycle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            LayoutCycle::Stacking => write!(f, "stacking"),
            LayoutCycle::Tabbed => write!(f, "tabbed"),
            LayoutCycle::Split => write!(f, "split"),
            LayoutCycle::SplitH => write!(f, "splith"),
            LayoutCycle::SplitV => write!(f, "splitv"),
            LayoutCycle::All => write!(f, "all"),
        }
    }
}

impl Display for LayoutCycleSingle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { <LayoutCycleSingle as Into<LayoutCycle>>::into(self.clone()).fmt(f) }
}

impl Display for LayoutCycleMulti {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { <LayoutCycleMulti as Into<LayoutCycle>>::into(self.clone()).fmt(f) }
}


impl Display for Split {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Split::Horizontal => write!(f, "horizontal"),
            Split::Vertical => write!(f, "vertical"),
            Split::None => write!(f, "none"),
        }
    }
}

impl Display for Bindsym {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Bindsym::WholeWindow => write!(f, "--whole-window"),
            Bindsym::Border => write!(f, "--border"),
            Bindsym::ExcludeTitlebar => write!(f, "--exclude-titlebar"),
            Bindsym::Release => write!(f, "--release"),
            Bindsym::Locked => write!(f, "--locked"),
            Bindsym::ToCode => write!(f, "--to-code"),
            Bindsym::InputDevice(device) => write!(f, "--input-device={}", device),
            Bindsym::NoWarn => write!(f, "--no-warn"),
            Bindsym::NoRepeat => write!(f, "--no-repeat"),
            Bindsym::Inhibited => write!(f, "--inhibited"),
        }
    }
}