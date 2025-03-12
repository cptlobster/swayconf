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
use subenum::subenum;
use crate::sway::options::ArgList;

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
    Multi(ArgList<LayoutCycleMulti>)
}

#[subenum(ConfigLayout)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Layout {
    #[subenum(ConfigLayout)]
    Default,
    #[subenum(ConfigLayout)]
    Stacking,
    #[subenum(ConfigLayout)]
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