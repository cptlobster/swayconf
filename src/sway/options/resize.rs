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
use crate::sway::options::Units;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum ResizeParams {
    #[strum(serialize = "grow {0}")]
    Grow(SingleAxisParams),
    #[strum(serialize = "shrink {0}")]
    Shrink(SingleAxisParams),
    #[serde(untagged)]
    #[strum(serialize = "set {0}")]
    Set (ResizeSetParams)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum SingleAxisParams {
    #[strum(serialize = "width {width} {unit}")]
    Width{
        #[serde(alias = "x")]
        width: u8,
        #[serde(default)]
        unit: Units
    },
    #[strum(serialize = "height {height} {unit}")]
    Height{
        #[serde(alias = "y")]
        height: u8,
        #[serde(default)]
        unit: Units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum ResizeSetParams{
    #[strum(serialize = "width {width} {unit}")]
    Width{
        #[serde(alias = "x")]
        width: u8,
        #[serde(default)]
        unit: Units
    },
    #[strum(serialize = "height {height} {unit}")]
    Height{
        #[serde(alias = "y")]
        height: u8,
        #[serde(default)]
        unit: Units
    },
    #[strum(serialize = "width {width} {unit} height {height} {unit}")]
    Both{
        #[serde(alias = "x")]
        width: u8,
        #[serde(alias = "y")]
        height: u8,
        #[serde(default)]
        unit: Units
    },
}