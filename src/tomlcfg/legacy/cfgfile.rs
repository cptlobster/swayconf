// Parses a config TOML file and converts to a Swayconf config file.
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

use std::path::PathBuf;
use toml::{Table, Value};
use crate::tomlcfg::legacy::base::{find, find_opt, table};
use crate::as_type_opt;
use crate::sway::commands::Config;
use crate::tomlcfg::legacy::{ParseResult, ParseError};
use crate::tomlcfg::legacy::config::{parse_bindsym_nokeys, parse_exec, parse_exec_always, 
                                     parse_include, parse_bar, gen_set};
use crate::sway::config::ConfigFile;

fn parse_includes(table: &Table) -> ParseResult<Vec<Config>> {
    let arr = as_type_opt!(find_opt(table, "include".to_string()), Value::Array)?;
    match arr {
        Some(a) => a.into_iter().map(parse_include).collect::<ParseResult<Vec<Config>>>(),
        None => Ok(vec![])
    }
}

fn parse_execs(table: &Table) -> ParseResult<Vec<Config>> {
    let arr = as_type_opt!(find_opt(table, "exec".to_string()), Value::Array)?;
    match arr {
        Some(a) => a.into_iter().map(parse_exec).collect::<ParseResult<Vec<Config>>>(),
        None => Ok(vec![])
    }
}

fn parse_execs_always(table: &Table) -> ParseResult<Vec<Config>> {
    let arr = as_type_opt!(find_opt(table, "exec-always".to_string()), Value::Array)?;
    match arr {
        Some(a) => a.into_iter().map(parse_exec_always).collect::<ParseResult<Vec<Config>>>(),
        None => Ok(vec![])
    }
}

fn parse_bindsyms(t: &Table) -> ParseResult<Vec<Config>> {
    let tab = table(t, "bindsym".to_string())?;
    tab.keys().map(|keystr| {
        let keys = keystr.clone().split("+").map(|a| a.to_string()).collect();
        parse_bindsym_nokeys(keys, find(tab, keystr.clone())?)
    }).collect::<ParseResult<Vec<Config>>>()
}

fn parse_sets(t: &Table) -> ParseResult<Vec<Config>> {
    let tab = table(t, "set".to_string())?;
    tab.keys().map(|keystr| {
        let name = keystr.clone();
        gen_set(name, find(tab, keystr.clone())?)
    }).collect::<ParseResult<Vec<Config>>>()
}

fn parse_bars(table: &Table) -> ParseResult<Option<Config>> {
    match find_opt(table, "bar".to_string()) {
        Some(t) => Ok(Some(parse_bar(t)?)),
        None => Ok(None)
    }
}

pub fn asm_config(path: PathBuf, table: &Table) -> ParseResult<ConfigFile> {
    let mut sets = parse_sets(table)?;
    let mut includes = parse_includes(table)?;
    let mut execs = parse_execs(table)?;
    let mut execs_always = parse_execs_always(table)?;
    let mut bindsyms = parse_bindsyms(table)?;
    let bar = parse_bars(table)?;

    let mut commands = vec![];

    // generate the swayconf config header
    let mut header = vec![
        "This configuration file was generated using the swayconf automated configurator.",
        "Please note that some sway features may not be fully supported.",
        "",
        "If you need any additional functionality that is not provided in swayconf, you can",
        "add it to this file, just note that it may be lost if you regenerate this file.",
        "",
        "For more information on this project, please visit https://github.com/cptlobster/swayconf"
    ].iter().map(|&s| Config::Comment(s.to_string())).collect();

    commands.append(&mut header);
    commands.push(Config::Blank);

    if !sets.is_empty() {
        header = vec![
            "Sway variables used in the [set] table"
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.append(&mut sets);
        commands.push(Config::Blank)
    }

    if !includes.is_empty() {
        header = vec![
            "Other files included in the [include] array",
            "Note: if any TOML files were included, you will need to generate those as well."
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.append(&mut includes);
        commands.push(Config::Blank)
    }

    if !execs.is_empty() {
        header = vec![
            "Autostart applications in the [exec] array",
            "Note: These will NOT run on a config reload"
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.append(&mut execs);
        commands.push(Config::Blank)
    }

    if !execs_always.is_empty() {
        header = vec![
            "Autostart applications in the [exec-always] array",
            "These will run again on a config reload"
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.append(&mut execs_always);
        commands.push(Config::Blank)
    }

    if !bindsyms.is_empty() {
        header = vec![
            "Keybinds provided in the [bindsym] table",
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.append(&mut bindsyms);
        commands.push(Config::Blank)
    }

    if bar.is_some() {
        header = vec![
            "swaybar configuration provided in the [bindsym] table",
        ].iter().map(|&s| Config::Comment(s.to_string())).collect();
        commands.append(&mut header);
        commands.push(bar.unwrap());
    }

    Ok(ConfigFile::new(path, commands))
}