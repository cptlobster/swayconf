/// Parses structure of options in TOML and converts to Swayconf structs
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

use toml::value::Value;
use crate::sway::options::{Size, Split, TogglableBool, Units};
use crate::tomlcfg::{ParseResult, ParseError};
use crate::tomlcfg::base::find_opt;
use crate::{one_of_type, as_type, as_type_opt};

pub fn parse_togglable_bool(value: &Value) -> ParseResult<TogglableBool> {
    fn match_togglable_bool_str(value: &String) -> ParseResult<TogglableBool> {
        match value.as_str() {
            "yes" | "enable" | "true" => Ok(TogglableBool::Enable),
            "no" | "disable" | "false" => Ok(TogglableBool::Disable),
            "toggle" => Ok(TogglableBool::Toggle),
            e => Err(ParseError::StringMismatch(vec!["enable".to_string(), "disable".to_string(), "toggle".to_string()], e.to_string())),
        }
    }
    
    fn match_togglable_bool_bool(value: &bool) -> ParseResult<TogglableBool> {
        if *value {Ok(TogglableBool::Enable)} else {Ok(TogglableBool::Disable)}
    }
    
    one_of_type!(value,
        Value::String, match_togglable_bool_str,
        Value::Boolean, match_togglable_bool_bool
    )
}

pub fn parse_size(value: &String) -> ParseResult<Size> {
    match value.as_str() {
        "grow" => Ok(Size::Grow),
        "shrink" => Ok(Size::Shrink),
        e => Err(ParseError::StringMismatch(vec!["grow".to_string(), "shrink".to_string()], e.to_string())),
    }
}

pub fn parse_units(value: &Value) -> ParseResult<Units> {
    let table = as_type!(value, Value::Table)?;
    let px = *as_type_opt!(find_opt(table, "px".to_string()), Value::Boolean)?.unwrap_or(&false);
    let ppt = *as_type_opt!(find_opt(table, "ppt".to_string()), Value::Boolean)?.unwrap_or(&true);

    if px == ppt {
        Err(ParseError::ConflictDiff("px".to_string(), "ppt".to_string()))
    } else {
        if px { Ok(Units::Px) } else { Ok(Units::Ppt) }
    }
}

pub fn to_u8(input: Option<&i64>) -> Option<u8> {
    match input {
        Some(n) => Some(*n as u8),
        None => None,
    }
}

pub fn parse_splitopt(value: &String) -> ParseResult<Split> {
    match value.as_str() {
        "h" | "horizontal" => Ok(Split::Horizontal),
        "v" | "vertical" => Ok(Split::Vertical),
        "none" => Ok(Split::None),
        e => Err(ParseError::StringMismatch(vec!["horizontal".to_string(), "vertical".to_string(), "none".to_string()], e.to_string())),
    }
}