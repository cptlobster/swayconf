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

use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use strum::Display;

/// Flags for bindsym commands.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Bind {
    #[strum(to_string = "--whole-window")]
    WholeWindow,
    #[strum(to_string = "--border")]
    Border,
    #[strum(to_string = "--exclude-titlebar")]
    ExcludeTitlebar,
    #[strum(to_string = "--release")]
    Release,
    #[strum(to_string = "--locked")]
    Locked,
    #[strum(to_string = "--to-code")]
    ToCode,
    #[strum(to_string = "--input-device={0}")]
    InputDevice(String),
    #[strum(to_string = "--no-warn")]
    NoWarn,
    #[strum(to_string = "--no-repeat")]
    NoRepeat,
    #[strum(to_string = "--inhibited")]
    Inhibited,
}

/// Key sequence for bindsym commands.
/// 
/// This exists mainly to provide [Display] support (similar to the [ArgList] struct), except
/// instead of joining everything with spaces it joins them with `+`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BindKeys(Vec<String>);

impl Display for BindKeys {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0.join("+"))
    }
}

impl Default for BindKeys {
    fn default() -> Self {
        BindKeys(Vec::new())
    }
}

impl BindKeys {
    pub fn new() -> Self {
        BindKeys::default()
    }

    pub fn from(vec: Vec<String>) -> Self {
        BindKeys(vec)
    }
}

/// Key sequence for bindcode commands.
///
/// This exists mainly to provide [Display] support (similar to the [ArgList] struct), except
/// instead of joining everything with spaces it joins them with `+`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BindCodes(Vec<u8>);

impl Display for BindCodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0.iter().map(|a|a.to_string()).collect::<Vec<String>>().join("+"))
    }
}

impl Default for BindCodes {
    fn default() -> Self {
        BindCodes(Vec::new())
    }
}

impl BindCodes {
    pub fn new() -> Self {
        BindCodes::default()
    }

    pub fn from(vec: Vec<u8>) -> Self {
        BindCodes(vec)
    }
}