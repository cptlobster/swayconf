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

/// Configuration structure for autogen functionality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Autogen {
    /// Startup applications (automatically run with `exec --no-startup-id`)
    startup: Vec<String>,
    /// Auto-generated bindsyms
    bindsym: bool,
    /// Generating workspaces. This specifies what key(s) correspond to each workspace; you will
    /// specify commands under `autogen.bindsym.workspaces`
    workspaces: Vec<String>
}