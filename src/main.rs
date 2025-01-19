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

/// Struct-based representation for Sway config files.
///
/// # Don't panic!
/// This module may be a bit convoluted and hard to understand at first, but don't panic! This will
/// make perfect sense if you **DON'T TOUCH THE ACTUAL CODE. I AM SERIOUS, DO NOT TRY TO MAKE THESE
/// YOURSELF.** This hellscape exists so that [serde] can nicely parse everything from a TOML file,
/// and so that [strum] can handle generating the appropriate commands. In the event that you *do*
/// need to dive into the guts of this code, good luck.
///
/// # Implementing Runtime Commands
/// In the event that you need to implement more commands, the following tips may be useful:
/// 1. Derive/macro AS MUCH AS POSSIBLE. [serde] and [strum] are incredibly forgiving with their
///    derived traits, and simply deriving [Serialize](serde::Serialize) and
///    [Deserialize](serde::Deserialize) (and for enums, [Display](strum::Display)) will take you a
///    *very* long way.
/// 2. Keep things consistent, please. All TOML fields should be interpreted as `kebab-case` and
///    (most) sway stuff is in `snake_case` (except for arguments, like those for `bindsym` or
///    `exec`). [serde] and [strum] will automatically handle those configurations if you annotate
///    properly:
///    ```
///    #[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
///    #[strum(serialize_all = "snake_case")]
///    ```
///    and then all struct and enum names can follow normal Rust conventions while everything else
///    serializes nicely to their respective formats.
/// 3. Make sure struct/enum variant names correspond as closely to their Sway names as possible.
///    You can add additional representations in TOML using [serde]'s `#[serde(alias = "")]`
///    annotation.
/// 
/// Full documentation on the actual effects of these commands is available in the sway(5) manpage.
mod sway;
/// TOML config mapping (deprecated)
// TODO: DELETE THIS MODULE
mod tomlcfg;

use std::path::{Path, PathBuf};
use tomlcfg::legacy::base::read;
use tomlcfg::legacy::cfgfile::asm_config;
use crate::sway::legacy::config::{ConfigFile, WritableConfig};
use crate::tomlcfg::legacy::ParseResult;

fn gen_conf(path: PathBuf) -> ParseResult<ConfigFile> {
    let cfg = read(path)?;
    asm_config(Path::new("samples/config").to_path_buf(), &cfg)
}

/// Main entrypoint
// TODO: rewrite so that it doesn't use the legacy module
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
