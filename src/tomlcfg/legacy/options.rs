use toml::Table;
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
use crate::sway::legacy::options::{Bindsym, Directional, FocusSibling, Hierarchy, Layout, 
                                   LayoutCycleMulti, LayoutCycleSingle, Size, Split, TogglableBool,
                                   Units};
use crate::tomlcfg::legacy::{ParseResult, ParseError};
use crate::tomlcfg::legacy::base::find_opt;
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

pub fn to_u8(input: &i64) -> u8 { *input as u8 }

pub fn to_u8_opt(input: Option<&i64>) -> Option<u8> {
    match input {
        Some(n) => Some(to_u8(n)),
        None => None,
    }
}

pub fn to_i8(input: &i64) -> i8 { *input as i8 }

pub fn to_i8_opt(input: Option<&i64>) -> Option<i8> {
    match input {
        Some(n) => Some(to_i8(n)),
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

pub fn parse_directional(value: &String) -> ParseResult<Directional> {
    match value.as_str() {
        "up" => Ok(Directional::Up),
        "down" => Ok(Directional::Down),
        "left" => Ok(Directional::Left),
        "right" => Ok(Directional::Right),
        e => Err(ParseError::StringMismatch(vec!["up".to_string(), "down".to_string(), "left".to_string(), "right".to_string()], e.to_string())),
    }
}

pub fn parse_sibling(value: &String) -> ParseResult<FocusSibling> {
    match value.as_str() { 
        "prev" | "previous" => Ok(FocusSibling::Prev),
        "next" => Ok(FocusSibling::Next),
        e => Err(ParseError::StringMismatch(vec!["prev".to_string(), "next".to_string()], e.to_string())),
    }
}

pub fn parse_hierarchy(value: &String) -> ParseResult<Hierarchy> {
    match value.as_str() {
        "child" => Ok(Hierarchy::Child),
        "parent" => Ok(Hierarchy::Parent),
        e => Err(ParseError::StringMismatch(vec!["parent".to_string(), "child".to_string()], e.to_string())),
    }
}

pub fn parse_layoutopt(value: &String) -> ParseResult<Layout> {
    match value.as_str() { 
        "default" => Ok(Layout::Default),
        "stacking" => Ok(Layout::Stacking),
        "tabbed" => Ok(Layout::Tabbed),
        "splith" | "split-h" => Ok(Layout::SplitH),
        "splitv" | "split-v" => Ok(Layout::SplitV),
        e => Err(ParseError::StringMismatch(vec!["default".to_string(), "stacking".to_string(), "tabbed".to_string(), "splith".to_string(), "splitv".to_string()], e.to_string())),
    }
}

pub fn parse_layoutcyclesingle(value: &String) -> ParseResult<LayoutCycleSingle> {
    match value.as_str() {
        "all" => Ok(LayoutCycleSingle::All),
        "split" => Ok(LayoutCycleSingle::Split),
        e => Err(ParseError::StringMismatch(vec!["all".to_string(), "split".to_string()], e.to_string())),
    }
}

pub fn parse_layoutcyclemulti(value: &String) -> ParseResult<LayoutCycleMulti> {
    match value.as_str() {
        "stacking" => Ok(LayoutCycleMulti::Stacking),
        "tabbed" => Ok(LayoutCycleMulti::Tabbed),
        "split" => Ok(LayoutCycleMulti::Split),
        "splith" | "split-h" => Ok(LayoutCycleMulti::SplitH),
        "splitv" | "split-v" => Ok(LayoutCycleMulti::SplitV),
        e => Err(ParseError::StringMismatch(vec!["stacking".to_string(), "tabbed".to_string(), "split".to_string(), "split_h".to_string(), "split_v".to_string()], e.to_string())),
    }
}

pub fn collect_bindsym_args(value: &Table) -> ParseResult<Vec<Bindsym>> {
    let args = vec!["whole-window", "border", "exclude-titlebar", "release", "locked",
        "to-code", "no-warn", "no-repeat", "inhibited"];
    let enums = vec![Bindsym::WholeWindow, Bindsym::Border, Bindsym::ExcludeTitlebar,
        Bindsym::Release, Bindsym::Locked, Bindsym::ToCode, Bindsym::NoWarn, Bindsym::NoRepeat,
        Bindsym::Inhibited];
    
    let mut arg_collection = vec![];
    
    for (a, e) in args.iter().zip(enums) {
        match find_opt(value, a.to_string()) {
            Some(value) => if *as_type!(value, Value::Boolean)? {arg_collection.push(e)},
            None => continue,
        };
    }
    
    match find_opt(value, "input-device".to_string()) {
        Some(value) => {
            let device = as_type!(value, Value::String)?.clone();
            arg_collection.push(Bindsym::InputDevice(device));
        }
        None => {}
    }
    
    Ok(arg_collection)
}