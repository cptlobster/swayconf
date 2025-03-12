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
use crate::sway::options::{ArgMap};

#[derive(PartialEq, Eq, Clone, Debug, Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case", untagged)]
pub enum ExecParams {
    #[strum(serialize = "{0}")]
    String(String),
    #[strum(serialize = "{args}{command}")]
    Flagged {
        #[serde(default, flatten)]
        args: ArgMap<Exec>,
        command: String
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Exec {
    #[strum(serialize = "--no-startup-id")]
    NoStartupId
}
