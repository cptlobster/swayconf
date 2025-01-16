/// Parses structure of config commands in TOML and converts to Swayconf structs
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

use std::ffi::OsStr;
use std::path::PathBuf;
use crate::tomlcfg::{ParseResult, ParseError};
use crate::tomlcfg::base::{find, find_opt, table};
use crate::tomlcfg::options::{collect_bindsym_args};
use crate::{as_type, as_type_opt};
use crate::sway::commands::{Config};
use toml::{Value};
use crate::tomlcfg::runtime::parse_runtime;

pub fn parse_exec(value: &Value) -> ParseResult<Config> {
    let table = as_type!(value, Value::Table)?;
    let mut command = as_type!(find(table, "command".to_string())?, Value::String)?.clone();
    let nsid = *as_type_opt!(find_opt(table, "no-startup-id".to_string()), Value::Boolean)?.unwrap_or(&false);

    if nsid { command = format!("--no-startup-id {}", command); };
    Ok(Config::Exec(command))
}

pub fn parse_exec_always(value: &Value) -> ParseResult<Config> {
    let table = as_type!(value, Value::Table)?;
    let mut command = as_type!(find(table, "command".to_string())?, Value::String)?.clone();
    let nsid = *as_type_opt!(find_opt(table, "no-startup-id".to_string()), Value::Boolean)?.unwrap_or(&false);

    if nsid { command = format!("--no-startup-id {}", command); };
    Ok(Config::ExecAlways(command))
}

pub fn parse_bindsym_nokeys(keys: Vec<String>, value: &Value) -> ParseResult<Config> {
    let t = as_type!(value, Value::Table)?;
    let command = Box::new(parse_runtime(t)?);
    let flags = collect_bindsym_args(t)?;
    Ok(Config::Bindsym{ keys, flags, command })
}

pub fn parse_bindsym(value: &Value) -> ParseResult<Config> {
    let t = as_type!(value, Value::Table)?;
    let keys = as_type!(find(t, "keys".to_string())?, Value::String)?.split("+").map(|a| a.to_string()).collect();
    parse_bindsym_nokeys(keys, value)
}

pub fn gen_set(name: String, v: &Value) -> ParseResult<Config> {
    let value = as_type!(v, Value::String)?.clone();

    Ok(Config::Set { name, value })
}

pub fn parse_set(value: &Value) -> ParseResult<Config> {
    let table = as_type!(value, Value::Table)?;
    let name = as_type!(find(table, "name".to_string())?, Value::String)?.clone();
    gen_set(name, value)
}

pub fn parse_bar(value: &Value) -> ParseResult<Config> {
    let table = as_type!(value, Value::Table)?;
    let bar_id = as_type_opt!(find_opt(table, "id".to_string()), Value::String)?.cloned().unwrap_or("".to_string());
    let subcommands = as_type!(find(table, "status-command".to_string())?, Value::String)?.clone();
    Ok(Config::Bar { bar_id, subcommands })
}

pub fn parse_kill(value: &Value) -> ParseResult<Config> {
    as_type!(value, Value::Boolean)?;
    Ok(Config::Kill)
}

pub fn parse_include(value: &Value) -> ParseResult<Config> {
    let pathstr = as_type!(value, Value::String)?;
    let path = PathBuf::from(pathstr);
    if path.extension() == Some(OsStr::new("toml")) {
        println!("WARN: This include references a TOML file. Make sure to generate this file via swayconf as well.");
        Ok(Config::Include(path.with_extension(OsStr::new(""))))
    } else {
        Ok(Config::Include(path))
    }
}