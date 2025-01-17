/// Command options and arguments
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
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Bindsym {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BindKeys(Vec<String>);

impl Deref for BindKeys {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BindArgs(Vec<Bindsym>);

impl Deref for BindArgs {
    type Target = Vec<Bindsym>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for BindArgs {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0
            .iter()
            .map(|a| format!("{a} "))
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl Default for BindArgs {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl BindArgs {
    pub fn new() -> Self {
        BindArgs::default()
    }
    
    pub fn from(vec: Vec<Bindsym>) -> Self {
        BindArgs(vec)
    }
}