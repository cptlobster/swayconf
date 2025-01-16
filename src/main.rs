/// Swayconf main entrypoint.
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
mod sway;
mod tomlcfg;

use std::path::{Path, PathBuf};
use tomlcfg::base::read;
use tomlcfg::cfgfile::asm_config;
use crate::sway::config::{ConfigFile, WritableConfig};
use crate::tomlcfg::ParseResult;

fn gen_conf(path: PathBuf) -> ParseResult<ConfigFile> {
    let cfg = read(path)?;
    asm_config(Path::new("samples/config").to_path_buf(), &cfg)
}

fn main() {
    env_logger::init();

    match gen_conf(PathBuf::from("./samples/config.toml")) {
        Ok(file) => {
            match file.write() {
                Ok(_) => log::info!("Successfully wrote to config file"),
                Err(e) => log::error!("Error writing config file: {}", e),
            }
        }
        Err(e) => { log::error!("Error generating config: {}", e); }
    }
}
