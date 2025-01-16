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
use crate::tomlcfg::base::{find, find_opt, table};
use crate::tomlcfg::options::{parse_togglable_bool, parse_size, parse_units, to_u8_opt, to_u8, 
                              to_i8, parse_splitopt, parse_directional, parse_sibling,
                              parse_hierarchy, parse_layoutopt, parse_layoutcyclesingle,
                              parse_layoutcyclemulti, collect_bindsym_args};
use crate::{one_of, as_type, as_type_opt, one_of_type};
use crate::sway::commands::{Runtime, SubFocus, SubLayout, SubMove};
use crate::sway::options::{FocusSibling, LayoutCycleMulti, RelWorkspace, Units};
use toml::{Table, Value};

// Parse a runtime command from a TOML table.
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
    fn pf_directional(value: &Value) -> ParseResult<Runtime> {
        let direction = parse_directional(as_type!(value, Value::String)?)?;
        Ok(Runtime::Focus(SubFocus::Directional(direction)))
    }
    
    fn pf_sibling(value: &Value) -> ParseResult<Runtime> {
        let sibling = parse_sibling(as_type!(value, Value::String)?)?;
        Ok(Runtime::Focus(SubFocus::Sibling(sibling)))
    }
    
    fn pf_hierarchy(value: &Value) -> ParseResult<Runtime> {
        let hierarchy = parse_hierarchy(as_type!(value, Value::String)?)?;
        Ok(Runtime::Focus(SubFocus::Hierarchy(hierarchy)))
    }
    
    fn pf_output(value: &Value) -> ParseResult<Runtime> {
        fn pf_named(value: &String) -> ParseResult<Runtime> {
            Ok(Runtime::Focus(SubFocus::OutputNamed(value.clone())))
        }
        
        fn pf_output_named(value: &Value) -> ParseResult<Runtime> {
            pf_named(as_type!(value, Value::String)?)
        }
        
        fn pf_output_directional(value: &Value) -> ParseResult<Runtime> {
            let direction = parse_directional(as_type!(value, Value::String)?)?;
            Ok(Runtime::Focus(SubFocus::OutputDirectional(direction)))
        }
        
        fn pf_table(value: &Table) -> ParseResult<Runtime> {
            one_of!(value,
                "directional", pf_output_directional,
                "named", pf_output_named
            )
        }
        
        one_of_type!(value,
            Value::String, pf_named,
            Value::Table, pf_table
        )
    }
    
    one_of!(as_type!(value, Value::Table)?,
        "directional", pf_directional,
        "sibling", pf_sibling,
        "hierarchy", pf_hierarchy,
        "output", pf_output
    )
}

fn parse_layout(value: &Value) -> ParseResult<Runtime> {
    fn parse_set(value: &Value) -> ParseResult<Runtime> {
        let layout = parse_layoutopt(as_type!(value, Value::String)?)?;
        Ok(Runtime::Layout(SubLayout::Set(layout)))
    }
    fn parse_cycle(value: &Value) -> ParseResult<Runtime> {
        fn parse_lcs(value: &String) -> ParseResult<Runtime> {
            let arg = parse_layoutcyclesingle(value)?;
            Ok(Runtime::Layout(SubLayout::Cycle(arg)))
        }
        fn parse_lcm(value: &Vec<Value>) -> ParseResult<Runtime> {
            let args = value.iter()
                .map(|m| as_type!(m, Value::String))
                .collect::<ParseResult<Vec<&String>>>()?
                .iter().cloned()
                .map(parse_layoutcyclemulti)
                .collect::<ParseResult<Vec<LayoutCycleMulti>>>()?;
            Ok(Runtime::Layout(SubLayout::CycleList(args)))
        }
        one_of_type!(value,
            Value::String, parse_lcs,
            Value::Array, parse_lcm
        )
    }
    one_of!(as_type!(value, Value::Table)?,
        "set", parse_set,
        "cycle", parse_cycle
    )
}

fn parse_move(value: &Value) -> ParseResult<Runtime> {
    fn pm_directional(value: &Value) -> ParseResult<Runtime> {
        let direction = parse_directional(as_type!(value, Value::String)?)?;
        Ok(Runtime::Move(SubMove::Directional{ direction, px: None }))
    }
    
    fn pm_coords(value: &Value) -> ParseResult<Runtime> {
        let table = as_type!(value, Value::Table)?;
        let x = to_i8(as_type!(find(table, "x".to_string())?, Value::Integer)?);
        let y = to_i8(as_type!(find(table, "x".to_string())?, Value::Integer)?);
        let x_unit = Units::Px;
        let y_unit = Units::Px;
        let absolute = *as_type!(find(table, "absolute".to_string())?, Value::Boolean)?;
        Ok(Runtime::Move(SubMove::Coordinates { x, y, x_unit, y_unit, absolute }))
    }
    
    fn pm_center(value: &Value) -> ParseResult<Runtime> {
        fn pm_c_bool(value: &bool) -> ParseResult<&bool> {
            Ok(value)
        }
        
        fn pm_c_tab(value: &Table) -> ParseResult<&bool> {
            as_type!(find(value, "absolute".to_string())?, Value::Boolean)
        }
        
        let absolute = *one_of_type!(value,
            Value::Boolean, pm_c_bool,
            Value::Table, pm_c_tab
        )?;
        Ok(Runtime::Move(SubMove::Center { absolute }))
    }
    
    fn pm_cursor(value: &Value) -> ParseResult<Runtime> {
        as_type!(value, Value::Boolean)?;
        Ok(Runtime::Move(SubMove::ToCursor))
    }
    
    fn pm_workspace(value: &Value) -> ParseResult<Runtime> {
        fn pm_w_tab(value: &Table) -> ParseResult<Runtime> {
            fn pm_w_output(value: &Value) -> ParseResult<Runtime> {
                match as_type!(value, Value::String)?.as_str() {
                    "prev" | "previous" => Ok(Runtime::Move(SubMove::ToWorkspaceOnOutput(FocusSibling::Prev))),
                    "next" => Ok(Runtime::Move(SubMove::ToWorkspaceOnOutput(FocusSibling::Next))),
                    s => Err(ParseError::StringMismatch(vec!["prev".to_string(), "next".to_string()], s.to_string())),
                }
            }
            fn pm_w_bf(value: &Value) -> ParseResult<Runtime> {
                as_type!(value, Value::Boolean)?;
                Ok(Runtime::Move(SubMove::BackAndForth))
            }
            
            one_of!(value,
                "output", pm_w_output,
                "back-and-forth", pm_w_bf
            )
        }
        
        fn pm_w_str(value: &String) -> ParseResult<Runtime> {
            match value.as_str() {
                "prev" | "previous" => Ok(Runtime::Move(SubMove::ToWorkspace(RelWorkspace::Prev))),
                "next" => Ok(Runtime::Move(SubMove::ToWorkspace(RelWorkspace::Next))),
                "current" => Ok(Runtime::Move(SubMove::ToWorkspace(RelWorkspace::Current))),
                "back-and-forth" => Ok(Runtime::Move(SubMove::BackAndForth)),
                s => Err(ParseError::StringMismatch(vec!["back-and-forth".to_string()], s.to_string())),
            }
        }
        
        one_of_type!(value,
            Value::Table, pm_w_tab,
            Value::String, pm_w_str
        )
    }
    
    fn pm_output(value: &Value) -> ParseResult<Runtime> {
        fn pm_named(value: &String) -> ParseResult<Runtime> {
            Ok(Runtime::Move(SubMove::ToNamedOutput(value.clone())))
        }

        fn pm_output_named(value: &Value) -> ParseResult<Runtime> {
            pm_named(as_type!(value, Value::String)?)
        }

        fn pm_output_directional(value: &Value) -> ParseResult<Runtime> {
            let direction = parse_directional(as_type!(value, Value::String)?)?;
            Ok(Runtime::Move(SubMove::ToDirectionalOutput(direction)))
        }

        fn pm_table(value: &Table) -> ParseResult<Runtime> {
            one_of!(value,
                "directional", pm_output_directional,
                "named", pm_output_named
            )
        }

        one_of_type!(value,
            Value::String, pm_named,
            Value::Table, pm_table
        )
    }

    one_of!(as_type!(value, Value::Table)?,
        "directional", pm_directional,
        "coordinates", pm_coords,
        "center", pm_center,
        "cursor", pm_cursor,
        "workspace", pm_workspace,
        "output", pm_output
    )
}

fn parse_reload(value: &Value) -> ParseResult<Runtime> {
    as_type!(value, Value::Boolean)?;
    Ok(Runtime::Reload)
}

fn parse_resize(value: &Value) -> ParseResult<Runtime> {
    let table = as_type!(value, Value::Table)?;
    let change = parse_size(as_type!(find(table, "change".to_string())?, Value::String)?)?;
    let x = to_u8_opt(as_type_opt!(find_opt(table, "x".to_string()), Value::Integer)?);
    let y = to_u8_opt(as_type_opt!(find_opt(table, "y".to_string()), Value::Integer)?);
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
    let t = as_type!(value, Value::Table)?;
    let keys = as_type!(find(t, "keys".to_string())?, Value::String)?.split("+").map(|a| a.to_string()).collect();
    let command = Box::new(parse_runtime(table(t, "command".to_string())?)?);
    let flags = collect_bindsym_args(t)?;
    Ok(Runtime::Bindsym{ keys, flags, command })
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
    fn parse_wsn(value: &i64) -> ParseResult<Runtime> {
        let number = to_u8(value);
        Ok(Runtime::Workspace{ number, name: None })
    }
    fn parse_wst(value: &Table) -> ParseResult<Runtime> {
        let number = to_u8(as_type!(find(value, "number".to_string())?, Value::Integer)?);
        let name = as_type_opt!(find_opt(value, "name".to_string()), Value::String)?.cloned();
        Ok(Runtime::Workspace{ number, name })
    }
    
    one_of_type!(value,
        Value::Integer, parse_wsn,
        Value::Table, parse_wst
    )
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
            "move.directional = \"down\"".to_string()
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