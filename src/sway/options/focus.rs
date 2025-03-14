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
use crate::sway::options;

/// The base level focus parameter enum. This will differentiate into one of the ~9 different
/// variants of the `focus` command in Sway when fully assembled.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum FocusParams {
    #[strum(serialize = "{0}")]
    Directional(options::Directional),
    #[strum(serialize = "{0}")]
    Hierarchy(options::Hierarchy),
    #[strum(serialize = "{0}")]
    Relative(options::Relative),
    #[strum(serialize = "{0} sibling")]
    Sibling(options::Relative),
    #[strum(serialize = "output {0}")]
    Output(FocusOutputOptions),
    Tiling,
    Floating,
    ModeToggle
}

/// Specific options for selecting focus outputs
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum FocusOutputOptions {
    #[strum(serialize = "{0}")]
    Directional(options::Directional),
    #[serde(untagged)]
    #[strum(serialize = "{0}")]
    Named(String)
}