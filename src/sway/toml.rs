use std::fs::read_to_string;
use std::path::PathBuf;
use toml::Table;
use crate::sway::config::ConfigFile;
use crate::sway::commands::{Config, Runtime};
use crate::sway::options;

fn read(filepath: PathBuf) -> Table {
    read_to_string(filepath).unwrap().parse().unwrap()
}

fn parse_to_cfg(table: &Table) -> Result<ConfigFile, String> {
    Err("Not Implemented".to_string())
}

fn parse_sets(table: &Table) -> Vec<Result<Config, String>> {
    table.keys().map(|key| {
        let name = key.to_string();
        match table.get(key) {
            Some(res) => {
                match res.try_into() {
                    Ok(value) => Ok(Config::Set { name, value }),
                    Err(e) => Err(format!("TOML parse error: {}", e)),
                }
            }
            None => {
                Err(format!("Key {} does not exist. This shouldn't be possible.", key))
            }
        }
    }).iter().collect()
}

fn parse_execs(tables: Vec<&Table>) -> Vec<Result<Config, String>> {
    for table in tables {
        match table.get("command") {
            Ok(c) => {
                match c.try_into() {
                    Ok(command) => Config::Exec(command),
                    Err(e) => Err(format!("TOML parse error: {}", e)),
                }
            }
            Err(_) => Err("Syntax error: command parameter missing".to_string()),
        }
    }
}

fn parse_exec_always(tables: Vec<&Table>) -> Vec<Result<Config, String>> {
    for table in tables {
        match table.get("command") {
            Ok(c) => {
                match c.try_into() {
                    Ok(command) => Config::ExecAlways(command),
                    Err(e) => Err(format!("TOML parse error: {}", e)),
                }
            }
            Err(_) => Err("Syntax error: command parameter missing".to_string()),
        }
    }
}

fn parse_bindsyms(table: &Table) -> Vec<Result<Config, String>> {
    table.keys().map(|k| {
        let keys = breakdown_keys(k.clone());
        match table.get(k).unwrap().try_into() {
            Some(res) => {
                let flags = get_bindsym_args(res);
                match match_runtime_command(res) {
                    Ok(c) => {
                        let command = Box::new(c);
                        Ok(Config::Bindsym { flags, keys, command })
                    },
                    Err(e) => Err(format!("Error parsing runtime command: {}", e)),
                }
            }
            None => Err(format!("Syntax error: bindsym {} malformed", k))
        }
    }).collect()
}

fn get_bindsym_args(table: &Table) -> Vec<options::Bindsym> {
    table.keys().filter_map(|k| match k.clone().as_str() {
        "whole-window" => Some(options::Bindsym::WholeWindow),
        "border" => Some(options::Bindsym::Border),
        "exclude-titlebar" => Some(options::Bindsym::ExcludeTitlebar),
        "release" => Some(options::Bindsym::Release),
        "locked" => Some(options::Bindsym::Locked),
        "to-code" => Some(options::Bindsym::ToCode),
        "input-device" => Some(options::Bindsym::InputDevice(String::new())),
        "no-warn" => Some(options::Bindsym::NoWarn),
        "no-repeat" => Some(options::Bindsym::NoRepeat),
        "inhibited" => Some(options::Bindsym::Inhibited),
        _ => None
    }).collect()
}

fn breakdown_keys(keys: String) -> Vec<String> { keys.split("+").collect() }

fn match_runtime_command(table: &Table) -> Result<Runtime, String> {
    let valid_cmds = table.keys().filter_map(|k| {
        match k.clone().as_str() {
            "exit" => Some(Runtime::Exit),
            "floating" => None,
            "focus" => None,
            "layout" => None,
            "move" => None,
            "reload" => Some(Runtime::Reload),
            "resize" => None,
            "split" => None,
            "bindsym" => None,
            "exec" => None,
            "exec-always" => None,
            "kill" => Some(Runtime::Kill),
            "workspace" => None,
            _ => None
        }
    }).collect();
    match valid_cmds.len() {
        1 => Ok(valid_cmds[0].clone()),
        _ => Err("Syntax error: One and only one runtime command must be declared".to_string()),
    }
}