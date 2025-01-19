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

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum PositionParams {
    Center,
    #[serde(alias = "mouse", alias = "pointer")]
    Cursor,
    #[serde(untagged)]
    #[strum(serialize = "{x} {unit} {y} {unit}")]
    Coordinates{
        x: u8,
        y: u8,
        #[serde(default)]
        unit: options::Units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum AbsolutePositionParams {
    #[strum(serialize = "{x} px {y} px")]
    Coordinates{ x: u8, y: u8 },
    Center,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveContainerParams {
    #[strum(serialize = "workspace {0}")]
    Workspace(MoveContainerToWorkspaceParams),
    #[strum(serialize = "output {0}")]
    Output(MoveToOutputParams),
    Scratchpad,
    #[strum(serialize = "mark {0}")]
    Mark(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveContainerToWorkspaceParams {
    #[strum(serialize = "{0}")]
    Relative(options::RelativeWorkspace),
    #[strum(serialize = "{0}_on_output")]
    OnOutput(options::Relative),
    BackAndForth,
    #[serde(untagged)]
    #[strum(to_string = "{0}")]
    Workspace(options::Workspace)
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum MoveToOutputParams {
    #[strum(serialize = "{0}")]
    Directional(options::Directional),
    Current,
    #[serde(untagged)]
    #[strum(serialize = "{0}")]
    Named(String)
}