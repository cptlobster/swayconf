/// Core functions for parsing TOML structures
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

use std::fs::read_to_string;
use std::path::PathBuf;
use crate::tomlcfg::{ParseError, ParseResult};
use toml::{Table, Value};

fn read(filepath: PathBuf) -> Table {
    read_to_string(filepath).unwrap().parse().unwrap()
}

fn find(input: ParseResult<&Table>, key: String) -> ParseResult<&Value> {
    match input {
        Ok(table) => match table.get(&key) {
            Some(value) => Ok(value),
            None => Err(ParseError::KeyNotFound(key)),
        }
        Err(e) => Err(e),
    }
}

fn as_table(input: ParseResult<&Value>) -> ParseResult<&Table> {
    match input {
        Ok(table) => match table.as_table() {
            Some(value) => Ok(value),
            None => Err(ParseError::IncorrectType(vec!["table".to_string()])),
        }
        Err(e) => Err(e),
    }
}

fn as_str(input: ParseResult<&Value>) -> ParseResult<String> {
    match input {
        Ok(table) => match table.as_str() {
            Some(value) => Ok(value.to_string()),
            None => Err(ParseError::IncorrectType(vec!["string".to_string()])),
        }
        Err(e) => Err(e),
    }
}

fn as_u8(input: ParseResult<&Value>) -> ParseResult<u8> {
    match input {
        Ok(table) => match table.as_integer() {
            Some(value) => match u8::try_from(value) {
                Ok(value) => Ok(value),
                Err(_) => Err(ParseError::IncorrectType(vec!["u8".to_string()])),
            },
            None => Err(ParseError::IncorrectType(vec!["u8".to_string()])),
        }
        Err(e) => Err(e),
    }
}

fn as_bool(input: ParseResult<&Value>) -> ParseResult<bool> {
    match input {
        Ok(table) => match table.as_bool() {
            Some(value) => Ok(value),
            None => Err(ParseError::IncorrectType(vec!["boolean".to_string()])),
        }
        Err(e) => Err(e),
    }
}

fn one_of(input: ParseResult<&Table>, keys: Vec<String>) -> ParseResult<(String, &Value)> {
    match input {
        Ok(table) => {
            let found_keys: Vec<String> = table.keys().cloned().filter(|k| keys.contains(k)).collect();
            match &found_keys.len() {
                1 => Ok((found_keys[0].clone(), table.get(&found_keys[0]).unwrap())),
                _ => Err(ParseError::MultiKey(found_keys)),
            }
        }
        Err(e) => Err(e),
    }
}