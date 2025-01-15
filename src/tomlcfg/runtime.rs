use crate::tomlcfg::base::find;
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
use crate::{one_of, as_type};
use crate::sway::commands::Runtime;
use toml::{Table, Value};

fn parse_runtime(table: &Table) -> ParseResult<Runtime> {
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
    Err(ParseError::NotImplemented)
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
    Err(ParseError::NotImplemented)
}

fn parse_resize(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_split(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_bindsym(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_exec(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_exec_always(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_kill(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_set(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}

fn parse_workspace(value: &Value) -> ParseResult<Runtime> {
    Err(ParseError::NotImplemented)
}