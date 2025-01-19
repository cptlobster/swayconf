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
use crate::sway::options;

/// The base level move parameter enum. This will differentiate into one of the ~14 different
/// variants of the `move` command in Sway when fully assembled.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveParams {
    #[strum(serialize = "{0}")]
    Directional(options::Directional),
    #[strum(serialize = "position {0}")]
    Position(PositionParams),
    #[serde(alias = "absolute")]
    #[strum(serialize = "absolute position {0}")]
    AbsolutePosition(AbsolutePositionParams),
    #[strum(serialize = "container to {0}")]
    Workspace(MoveContainerParams),
    #[strum(serialize = "workspace to output {0}")]
    Output(MoveToOutputParams),
}

/// Move to position variants.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum PositionParams {
    /// Move to the center of the current output
    Center,
    /// Center window on the cursor
    #[serde(alias = "mouse", alias = "pointer")]
    Cursor,
    /// Move the window to the specified position on the workspace
    #[serde(untagged)]
    #[strum(serialize = "{x} {unit} {y} {unit}")]
    Coordinates{
        x: u8,
        y: u8,
        #[serde(default)]
        unit: options::Units
    }
}

/// Move to absolute position variants. This is based on all monitors.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum AbsolutePositionParams {
    /// Move to the center of all output
    Center,
    /// Move to the specified position relative to all outputs
    #[serde(untagged)]
    #[strum(serialize = "{x} px {y} px")]
    Coordinates{ x: u8, y: u8 },
}

/// Move container variants.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveContainerParams {
    /// Move container to specified workspace
    #[strum(serialize = "workspace {0}")]
    Workspace(MoveContainerToWorkspaceParams),
    /// Move container to specified output
    #[strum(serialize = "output {0}")]
    Output(MoveToOutputParams),
    /// Move container to scratchpad
    Scratchpad,
    /// Move container to mark (defined using mark command)
    #[strum(serialize = "mark {0}")]
    Mark(String)
}

/// Move container to workspace variants.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveContainerToWorkspaceParams {
    /// Relative (in numeric order)
    #[strum(serialize = "{0}")]
    Relative(options::RelativeWorkspace),
    /// Relative on output (in numeric order)
    #[strum(serialize = "{0}_on_output")]
    OnOutput(options::Relative),
    /// Back to previously focused workspace
    BackAndForth,
    /// To numbered / named workspace
    #[serde(untagged)]
    #[strum(to_string = "{0}")]
    Workspace(options::Workspace)
}

/// Move container/workspace to output params.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveToOutputParams {
    /// Relative direction
    #[strum(serialize = "{0}")]
    Directional(options::Directional),
    /// Current output
    Current,
    /// Named output
    #[serde(untagged)]
    #[strum(serialize = "{0}")]
    Named(String)
}