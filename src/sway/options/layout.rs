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
use subenum::subenum;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum LayoutParams {
    #[strum(serialize = "{0}")]
    Set(Layout),
    #[strum(serialize = "toggle {0}")]
    Cycle(LayoutCycleParams)
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(untagged)]
#[strum(serialize_all = "kebab-case")]
pub enum LayoutCycleParams {
    #[strum(serialize = "{0}")]
    Single(LayoutCycleSingle),
    #[strum(serialize = "{0}")]
    Multi(Vec<LayoutCycleMulti>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Layout {
    Default,
    Stacking,
    Tabbed,
    #[serde(alias = "splith")]
    #[strum(serialize = "splith")]
    SplitH,
    #[serde(alias = "splitv")]
    #[strum(serialize = "splitv")]
    SplitV,
}

#[subenum(LayoutCycleSingle, LayoutCycleMulti)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum LayoutCycle {
    #[subenum(LayoutCycleMulti)]
    Stacking,
    #[subenum(LayoutCycleMulti)]
    Tabbed,
    #[subenum(LayoutCycleSingle, LayoutCycleMulti)]
    Split,
    #[subenum(LayoutCycleMulti)]
    #[serde(alias = "splith")]
    #[strum(serialize = "splith")]
    SplitH,
    #[subenum(LayoutCycleMulti)]
    #[serde(alias = "splitv")]
    #[strum(serialize = "splitv")]
    SplitV,
    #[subenum(LayoutCycleSingle)]
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LayoutCycleMultiList(Vec<LayoutCycleMulti>);

impl Deref for LayoutCycleMultiList {
    type Target = Vec<LayoutCycleMulti>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LayoutCycleMultiList {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0
            .iter()
            .map(|a| format!("{a} "))
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl Default for LayoutCycleMultiList {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl LayoutCycleMultiList {
    pub fn new() -> Self {
        LayoutCycleMultiList::default()
    }

    pub fn from(vec: Vec<LayoutCycleMulti>) -> Self {
        LayoutCycleMultiList(vec)
    }
}