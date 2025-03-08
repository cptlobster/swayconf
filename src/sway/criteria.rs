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
use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};
use strum::Display;

/// An array of values.
///
/// A [Vec] would normally suffice for our purposes, but this struct implements [Display],
/// [Default], and [Serialize]/[Deserialize] traits to be compatible with everything else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CriteriaVec(Vec<Criteria>);

impl FmtDisplay for CriteriaVec {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[{}]", self.0.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

impl Default for CriteriaVec {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl CriteriaVec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(lst: Vec<Criteria>) -> Self {
        Self(lst)
    }

    pub fn insert(&mut self, criteria: Criteria) {
        self.0.push(criteria);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Criteria {
    All,
    #[strum(to_string = "app_id=\"{0}\"")]
    AppId(String),
    #[strum(to_string = "class=\"{0}\"")]
    Class(String),
    #[strum(to_string = "con_id=\"{0}\"")]
    ConId(String),
    #[strum(to_string = "con_mark=\"{0}\"")]
    ConMark(String),
    Floating,
    #[strum(to_string = "id={0}")]
    Id(u32),
    #[strum(to_string = "instance=\"{0}\"")]
    Instance(String),
    #[strum(to_string = "pid={0}")]
    Pid(u32),
    #[strum(to_string = "shell_type=\"{0}\"")]
    Shell(ShellType),
    Tiling,
    #[strum(to_string = "title=\"{0}\"")]
    Title(String),
    Urgent,
    #[strum(to_string = "window_role=\"{0}\"")]
    WindowRole(String),
    #[strum(to_string = "window_type=\"{0}\"")]
    WindowType(String),
    #[strum(to_string = "workspace=\"{0}\"")]
    Workspace(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum ShellType {
    XdgShell,
    #[serde(rename = "xwayland")]
    #[strum(to_string = "xwayland")]
    XWayland,
    #[strum(to_string = "__focused__")]
    Focused
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum UrgentState {
    First,
    Last,
    Latest,
    Newest,
    Oldest,
    Recent
}