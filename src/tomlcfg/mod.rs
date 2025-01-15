/// TOML configuration parsing
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
mod core;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
enum ParseError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Incorrect type: Must be one of the following: ({})", .0.join(", "))]
    IncorrectType(Vec<String>),
    #[error("One and only one key must be provided: found ({})", .0.join(", "))]
    MultiKey(Vec<String>),
    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),
}

type ParseResult<T> = Result<T, ParseError>;