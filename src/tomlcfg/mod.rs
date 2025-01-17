/// TOML configuration parsing
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
pub mod legacy;
mod mappings;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::sway::legacy::commands::{Config, Runtime};
use crate::tomlcfg::mappings::{BindsymPart};
use crate::sway::legacy::options;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
enum TomlCfg {
    SingleFile {
        set: Option<HashMap<String, String>>,
        include: Option<Vec<Config>>,
        exec: Option<Vec<Config>>,
        exec_always: Option<Vec<Config>>,
        bindsym: Option<HashMap<String, BindsymPart>>,
        bar: Option<Config>
    },
    Tree {
        path: String,
        contents: Vec<TomlCfg>
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::sway::legacy::commands::SubMove;
    use crate::sway::legacy::options::Bindsym;
    use super::*;

    #[test]
    fn test_toml_config() {
        let mut passed = true;
        let mut print = |cmd: TomlCfg| {
            println!("{:?}", cmd);
            match toml::to_string(&cmd) {
                Ok(s) => { println!("{}", s); }
                Err(e) => {
                    println!("ERROR: {}", e);
                    passed = false;
                }
            }
        };

        let mut set = HashMap::new();
        set.insert(String::from("foo"), String::from("bar"));
        set.insert(String::from("baz"), String::from("shlonk"));

        let mut include = Vec::new();
        include.push(Config::Include(PathBuf::from("./config.toml")));
        include.push(Config::Include(PathBuf::from("./path/to/beans.toml")));

        let mut exec = Vec::new();
        exec.push(Config::Exec("ls -la ~".to_string()));
        exec.push(Config::Exec("systemctl start docker.service".to_string()));

        let mut bindsym = HashMap::new();
        bindsym.insert(String::from("Mod4+A"), BindsymPart(
            vec![],
            Runtime::Move(SubMove::ToWorkspace(options::RelWorkspace::Prev))
        ));
        bindsym.insert(String::from("Mod4+Shift+R"), BindsymPart(
            vec![Bindsym::Release],
            Runtime::Reload
        ));

        let bar = Config::Bar{ bar_id: "".to_string(), subcommands: "status_command i3blocks".to_string() };

        let tcfg = TomlCfg::SingleFile {
            set: Some(set),
            include: Some(include),
            exec: Some(exec),
            exec_always: None,
            bindsym: Some(bindsym),
            bar: Some(bar)
        };

        print(tcfg);

        assert!(passed);
    }
}