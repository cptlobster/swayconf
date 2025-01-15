use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::Error;
use toml::{Table, Value};

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