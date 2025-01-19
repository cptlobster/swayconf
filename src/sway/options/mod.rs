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

/// All structs for bindsym/bindcode commands
pub mod bind;
/// All structs for exec commands
pub mod exec;
/// All structs for layout commands
pub mod layout;
/// All structs for focus commands
pub mod focus;
/// All structs for move commands
pub mod mov;

use std::fmt::{Display as FmtDisplay, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use serde::de::{Visitor, Error, Unexpected, Deserializer};
use strum::Display;

/// Options used for togglable boolean commands.
///
/// Has true/false, and a "toggle" value to switch between true and false. You can represent it in
/// the following forms in source configuration files:
/// ```toml
/// enable = ["true", "yes", "enable", true]
/// disable = ["false", "no", "disable", false]
/// toggle = ["toggle"]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum TogglableBool {
    #[serde(alias = "true", alias = "yes")]
    Enable,
    #[serde(alias = "false", alias = "no")]
    Disable,
    Toggle
}

/// Options used for the `split` command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Split {
    #[serde(alias = "h")]
    Horizontal,
    #[serde(alias = "v")]
    Vertical,
    None,
}

/// Different forms of workspace command options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum Workspace {
    #[strum(serialize = "{0}")]
    Numeric(u8),
    #[strum(serialize = "{number} {name}")]
    Named{
        number: u8,
        #[serde(default)]
        name: String
    },
}

/// Options for parent/child hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Hierarchy {
    Parent,
    Child,
}

/// Options for sibling hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Relative {
    #[serde(alias = "previous")]
    Prev,
    Next,
}

/// Options for relative workspace commands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum RelativeWorkspace {
    #[serde(alias = "previous")]
    Prev,
    Next,
    Current,
}

/// Options for directional arguments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Directional {
    Up,
    Down,
    Left,
    Right,
}

/// Positional units
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "snake_case")]
pub enum Units {
    #[serde(alias = "pixels", alias = "pixel")]
    Px,
    #[serde(alias = "percent", alias = "%", alias = "points")]
    Ppt
}

impl Default for Units {
    fn default() -> Self { Units::Px }
}

/// An array of values.
///
/// A [Vec] would normally suffice for our purposes, but this struct implements [Display],
/// [Default], and [Serialize]/[Deserialize] traits to be compatible with everything else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArgList<T: FmtDisplay>(Vec<T>);

impl<T: FmtDisplay> FmtDisplay for ArgList<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0
            .iter()
            .map(|a| format!("{a} "))
            .collect::<Vec<String>>()
            .join(""))
    }
}

impl<T: FmtDisplay> Default for ArgList<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: FmtDisplay> ArgList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

// since serde doesn't offer an easy way to support deserializing multiple types into a single enum,
// we have to write our own `Visitor` and `Deserialize` traits for `TogglableBool`. This allows us
// to represent `TogglableBool`s as booleans or strings
impl<'de> Visitor<'de> for TogglableBool {
    type Value = TogglableBool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("true, false, or toggle")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value { Ok(TogglableBool::Enable) } else { Ok(TogglableBool::Disable) }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "true" | "yes" | "enable" => Ok(TogglableBool::Enable),
            "false" | "no" | "disable" => Ok(TogglableBool::Disable),
            "toggle" => Ok(TogglableBool::Toggle),
            _ => Err(Error::invalid_value(Unexpected::Str(value), &self))
        }
    }
}

impl<'de> Deserialize<'de> for TogglableBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_any::<TogglableBool>(TogglableBool::Disable)
    }
}