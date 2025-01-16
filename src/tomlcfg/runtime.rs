/// Parses structure of runtime commands in TOML and converts to Swayconf structs
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

use crate::tomlcfg::{ParseResult, ParseError};
use crate::tomlcfg::base::{find, find_opt};
use crate::tomlcfg::options::{parse_togglable_bool, parse_size, parse_units, to_u8, parse_splitopt};
use crate::{one_of, as_type, as_type_opt};
use crate::sway::commands::Runtime;
use toml::{Table, Value};

pub fn parse_runtime(table: &Table) -> ParseResult<Runtime> {
    one_of!(table,
        "exit", parse_exit,
        "floating", parse_floating,
        "focus", parse_focus,
        "layout", parse_layout,
        "move", parse_move,
        "reload", parse_reload,
        "resize", parse_resize,
        "split", parse_split,
        "bindsym", parse_bindsym,
        "exec", parse_exec,
        "exec-always", parse_exec_always,
        "kill", parse_kill,
        "set", parse_set,
        "workspace", parse_workspace
    )
}

fn parse_exit(value: &Value) -> ParseResult<Runtime> {
    as_type!(value, Value::Boolean)?;
    Ok(Runtime::Exit)
}

fn parse_floating(value: &Value) -> ParseResult<Runtime> {
    let tb = parse_togglable_bool(value)?;
    Ok(Runtime::Floating(tb))
}

fn parse_focus(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_layout(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_move(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_reload(value: &Value) -> ParseResult<Runtime> {
    as_type!(value, Value::Boolean)?;
    Ok(Runtime::Reload)
}

fn parse_resize(value: &Value) -> ParseResult<Runtime> {
    let table = as_type!(value, Value::Table)?;
    let change = parse_size(as_type!(find(table, "change".to_string())?, Value::String)?)?;
    let x = to_u8(as_type_opt!(find_opt(table, "x".to_string()), Value::Integer)?);
    let y = to_u8(as_type_opt!(find_opt(table, "y".to_string()), Value::Integer)?);
    let unit = parse_units(value)?;
    if x.is_some() == y.is_some() {
        Err(ParseError::ConflictDiff("x".to_string(), "y".to_string()))
    } else {
        Ok(Runtime::Resize { change, x, y, unit })
    }
}

fn parse_split(value: &Value) -> ParseResult<Runtime> {
    let split = parse_splitopt(as_type!(value, Value::String)?)?;
    Ok(Runtime::Split(split))
}

fn parse_bindsym(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_exec(value: &Value) -> ParseResult<Runtime> {
    let table = as_type!(value, Value::Table)?;
    let mut command = as_type!(find(table, "command".to_string())?, Value::String)?.clone();
    let nsid = *as_type_opt!(find_opt(table, "no-startup-id".to_string()), Value::Boolean)?.unwrap_or(&false);

    if nsid { command = format!("--no-startup-id {}", command); };
    Ok(Runtime::Exec(command))
}

fn parse_exec_always(value: &Value) -> ParseResult<Runtime> {
    let table = as_type!(value, Value::Table)?;
    let mut command = as_type!(find(table, "command".to_string())?, Value::String)?.clone();
    let nsid = *as_type_opt!(find_opt(table, "no-startup-id".to_string()), Value::Boolean)?.unwrap_or(&false);

    if nsid { command = format!("--no-startup-id {}", command); };
    Ok(Runtime::ExecAlways(command))
}

fn parse_kill(value: &Value) -> ParseResult<Runtime> {
    as_type!(value, Value::Boolean)?;
    Ok(Runtime::Kill)
}

fn parse_set(value: &Value) -> ParseResult<Runtime> {
    let table = as_type!(value, Value::Table)?;
    let name = as_type!(find(table, "name".to_string())?, Value::String)?.clone();
    let value = as_type!(find(table, "value".to_string())?, Value::String)?.clone();
    
    Ok(Runtime::Set{ name, value })
}

fn parse_workspace(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use crate::sway::commands::SubMove;
    use crate::sway::options::Directional;
    use crate::tomlcfg::base::from_str;
    use super::*;
    
    #[test]
    fn test_parse_runtime() {
        let teststrs = vec![
            "exit = true".to_string(),
            "exec = { command = \"ls -la ~\", no-startup-id = true }".to_string(),
            "set = { name = \"beans\", value = \"a\" }".to_string(),
            "move.direction = \"down\"".to_string()
        ];
        
        let expecteds = vec![
            Runtime::Exit,
            Runtime::Exec("--no-startup-id ls -la ~".to_string()),
            Runtime::Set{ name: "beans".to_string(), value: "a".to_string() },
            Runtime::Move(SubMove::Directional { direction: Directional::Down, px: None })
        ];
        
        for (teststr, expected) in teststrs.iter().zip(expecteds) {
            println!("TEST: {}", teststr);
            let result= from_str(teststr.clone()).and_then(|t| parse_runtime(&t));
            
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
            println!("OKAY: {}", expected.to_string());
        }
    }
}