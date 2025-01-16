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
use std::path::{PathBuf};
use crate::tomlcfg::{ParseError, ParseResult};
use toml::{Table, Value};

#[macro_export]
macro_rules! as_type {
    ($input:expr, $t:path) => {
        match $input {
            $t(v) => Ok(v),
            _ => Err(ParseError::IncorrectType(vec![stringify!($t).to_string()])),
        }
    };
}

#[macro_export]
macro_rules! one_of_type {
    ($input:expr, $($t:path, $target:ident), *) => {
        match $input {
            $($t(v) => $target(v),)*
            _ => Err(ParseError::IncorrectType(vec![$(stringify!($t).to_string(),)*])),
        }
    };
}

#[macro_export]
macro_rules! one_of {
    ($input:expr, $($key:literal, $target:ident), *) => {{
        let expected_keys = vec![$($key),*];
        let found_keys: Vec<String> = $input.keys().cloned().filter(|k| expected_keys.contains(&k.as_str())).collect();
        match &found_keys.len() {
            1 => match found_keys[0].as_str() {
                $($key => $target(find($input, $key.to_string())?),)*
                _ => Err(ParseError::MultiKey(vec![])),
            }
            _ => Err(ParseError::MultiKey(found_keys)),
        }
    }}
}

pub fn read(filepath: PathBuf) -> ParseResult<Table> {
    match read_to_string(filepath).unwrap().parse() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(ParseError::TomlError(error))
    }
}

pub fn from_str(str: String) -> ParseResult<Table> {
    match str.parse() {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(ParseError::TomlError(error))
    }
}

pub fn find(table: &Table, key: String) -> ParseResult<&Value> {
    match table.get(&key) {
        Some(value) => Ok(value),
        None => Err(ParseError::KeyNotFound(key)),
    }
}
pub fn table(input: &Table, key: String) -> ParseResult<&Table> {
    find(input, key).and_then(|table| as_type!(table, Value::Table))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let source = "a = 1\nb = \"two\"\nc = [true, false]\n[d]\none = 4".to_string();

        let base = from_str(source);

        match base {
            Ok(t) => {
                let res_a = find(&t, "a".to_string());
                let res_b = find(&t, "b".to_string());
                let res_e = find(&t, "e".to_string());
                assert!(res_a.is_ok());
                assert!(res_b.is_ok());
                assert!(res_e.is_err());
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}