use std::fs::read_to_string;
use std::path::PathBuf;
use toml::{Table, Value};
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

fn parse_execs_always(tables: Vec<&Table>) -> Vec<Result<Config, String>> {
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
            "exit" => Some(Ok(Runtime::Exit)),
            "floating" => Some(parse_floating(table.get("floating").unwrap())),
            "focus" => Some(parse_focus(table.get("focus").unwrap())),
            "layout" => Some(parse_layout(table.get("layout").unwrap())),
            "move" => Some(parse_move(table.get("move").unwrap())),
            "reload" => Some(Ok(Runtime::Reload)),
            "resize" => Some(parse_resize(table.get("resize").unwrap())),
            "split" => Some(parse_split(table.get("split").unwrap())),
            "exec" => Some(parse_exec(table.get("exec").unwrap())),
            "exec-always" => Some(parse_exec_always(table.get("exec").unwrap())),
            "kill" => Some(Ok(Runtime::Kill)),
            "workspace" => Some(parse_workspace(table.get("workspace").unwrap())),
            _ => None
        }
    }).collect();
    match valid_cmds.len() {
        1 => valid_cmds[0].clone(),
        _ => Err("Syntax error: One and only one runtime command must be declared".to_string()),
    }
}

fn parse_exec(table: &Table) -> Result<Runtime, String> {
    match table.get("command") {
        Ok(c) => {
            match c.try_into() {
                Ok(command) => Runtime::Exec(command),
                Err(e) => Err(format!("TOML parse error: {}", e)),
            }
        }
        Err(_) => Err("Syntax error: command parameter missing".to_string()),
    }
}

fn parse_exec_always(table: &Table) -> Result<Runtime, String> {
    match table.get("command") {
        Ok(c) => {
            match c.try_into() {
                Ok(command) => Runtime::ExecAlways(command),
                Err(e) => Err(format!("TOML parse error: {}", e)),
            }
        }
        Err(_) => Err("Syntax error: command parameter missing".to_string()),
    }
}

fn parse_floating(value: &Value) -> Result<Runtime, String> {
    let v = match value.as_bool() {
        Some(res) => Some(if res {options::TogglableBool::Enable} else {options::TogglableBool::Disable}),
        None => match value.as_str() {
            Some("toggle") => Some(options::TogglableBool::Toggle),
            Some("true") => Some(options::TogglableBool::Enable),
            Some("enable") => Some(options::TogglableBool::Enable),
            Some("false") => Some(options::TogglableBool::Disable),
            Some("disable") => Some(options::TogglableBool::Disable),
            _ => None
        }
    };
    match v {
        Some(v) => Ok(Runtime::Floating(v)),
        None => Err("Syntax error: floating parameter must match togglable boolean".to_string()),
    }
}

fn parse_focus(table: &Table) -> Result<Runtime, String> {
    Err("Not Implemented".to_string())
}

fn parse_move(table: &Table) -> Result<Runtime, String> {
    Err("Not Implemented".to_string())
}

fn parse_resize(table: &Table) -> Result<Runtime, String> {
    Err("Not Implemented".to_string())
}

fn parse_workspace(value: &Value) -> Result<Runtime, String> {
    match value.as_u8() {
        Some(number) => Ok(Runtime::Workspace { number, name: None }),
        None => {
            match value.as_table() {
                Some(t) => {
                    let number = t.get("number").unwrap().as_u8();
                    let name = t.get("name").unwrap().as_str();
                    Ok(Runtime::Workspace{ number, name })
                }
                None => Err("Syntax error: Workspace must be integer or table"),
            }
        }
    }
}

fn parse_split(table: &Table) -> Result<Runtime, String> {
    Err("Not Implemented".to_string())
}

fn parse_directional(value: &Value) -> Result<options::Directional, String> {
    match value.as_str() {
        Some("up") => Ok(options::Directional::Up),
        Some("down") => Ok(options::Directional::Down),
        Some("left") => Ok(options::Directional::Left),
        Some("right") => Ok(options::Directional::Right),
        Some(_) => Err("Syntax error: must be one of (up, down, left, right)".to_string()),
        None => Err("Syntax error: must be a string".to_string()),
    }
}

fn parse_focus_sibling(value: &Value) -> Result<options::FocusSibling, String> {
    match value.as_str() {
        Some("next") => Ok(options::FocusSibling::Next),
        Some("prev") => Ok(options::FocusSibling::Prev),
        Some(_) => Err("Syntax error: must be one of (next, prev)".to_string()),
        None => Err("Syntax error: must be a string".to_string()),
    }
}

fn parse_hierarchy(value: &Value) -> Result<options::Hierarchy, String> {
    match value.as_str() {
        Some("parent") => Ok(options::Hierarchy::Parent),
        Some("child") => Ok(options::Hierarchy::Child),
        Some(_) => Err("Syntax error: must be one of (parent, child)".to_string()),
        None => Err("Syntax error: must be a string".to_string()),
    }
}

fn parse_layout(table: &Table) -> Result<Runtime, String> {
    Err("Not Implemented".to_string())
}

fn parse_layout_opt(value: &Value) -> Result<options::Layout, String> {
    match value.as_str() {
        Some("tabbed") => Ok(options::Layout::Tabbed),
        Some("default") => Ok(options::Layout::Default),
        Some("split-h") => Ok(options::Layout::SplitH),
        Some("split-v") => Ok(options::Layout::SplitV),
        Some("stacking") => Ok(options::Layout::Stacking),
        Some(_) => Err("Syntax error: must be one of (default, stacking, tabbed, split-h, split-v)".to_string()),
        None => Err("Syntax error: must be a string".to_string()),
    }
}