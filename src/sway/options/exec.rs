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

use std::ops::Deref;
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(PartialEq, Eq, Clone, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case", untagged)]
pub enum ExecParams {
    #[strum(serialize = "{0}")]
    String(String),
    #[strum(serialize = "{args}{command}")]
    Flagged {
        #[serde(default)]
        args: ExecFlags,
        command: String
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Exec {
    #[strum(serialize = "--no-startup-id")]
    NoStartupId
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecFlags(Vec<Exec>);

impl Deref for ExecFlags {
    type Target = Vec<Exec>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ExecFlags {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0
            .iter()
            .map(|a| format!("{a} "))
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl Default for ExecFlags {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl ExecFlags {
    pub fn new() -> Self {
        ExecFlags::default()
    }

    pub fn from(vec: Vec<Exec>) -> Self {
        ExecFlags(vec)
    }
}