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
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Configure colors of window borders and title bars
pub enum ClientOpts {
    /// Ignored. Only present for i3 compatibility.
    Background(String),
    /// The window that has focus
    Focused(ClientColors),
    /// The most recently focused window in a container which isn't focused
    FocusedInactive(ClientColors),
    /// A view that has a focused descendant container
    FocusedTabTitle(ClientColors),
    /// Ignored. Only present for i3 compatibility.
    Placeholder(ClientColors),
    /// A view that does not have focus
    Unfocused(ClientColors),
    /// A view with an urgency hint.
    /// 
    /// *Note: Since native Wayland windows do not support urgency, this only works for XWayland
    /// windows.*
    Urgent(ClientColors)
}

/// All color groups for client classes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientColors {
    /// The border around the title bar
    border: String,
    /// The background of the title bar
    background: String,
    /// The text color of the title bar
    text: String,
    /// The color used to indicate where new views will open
    indicator: Option<String>,
    /// The border around the view itself
    child_border: Option<String>
}

impl Display for ClientColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (self.indicator.clone(), self.child_border.clone()) {
            (Some(ind), Some(cb)) => write!(f, "{} {} {} {} {}", self.border, self.background, self.text, ind, cb),
            (Some(ind), None) => write!(f, "{} {} {} {}", self.border, self.background, self.text, ind),
            (None, _) => write!(f, "{} {} {}", self.border, self.background, self.text),
        }
    }
}