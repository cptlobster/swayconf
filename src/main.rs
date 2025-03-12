// <one line to give the program's name and a brief idea of what it does.>
// Copyright (C) 2024, 2025 Dustin Thomas <stdio@cptlobster.dev>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//

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

use std::fs;
use std::fs::File;
use std::io::{Error as IoError, Write};
use toml::de::Error as TomlError;
use std::path::{PathBuf};
use std::process::Command;
use thiserror::Error;
use sway::config::Config;
use derive_more::{From};
use clap::Parser;
use clio::{InputPath, OutputPath, ClioPath};

/// Configuration generator for the Sway window manager.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The TOML file to read from. Defaults to "./config.toml" if unspecified.
    #[arg(short, long, value_parser, default_value = "./config.toml")]
    input_file: InputPath,
    /// The location to output the Sway config file to. If unspecified, uses the same path as the
    /// input file, but with the ".toml" extension stripped.
    #[arg(short, long, value_parser)]
    output_file: Option<OutputPath>,
    /// Reload Sway if enabled. This can be used if you are writing directly to your Sway config
    /// files.
    #[arg(short, long, default_value = "false")]
    reload: bool,
}

#[derive(Debug, Error, From)]
enum SwayconfError {
    #[error("I/O Error: {0}")]
    Io(IoError),
    #[error("Config Parse Error: {0}")]
    Toml(TomlError),
}

fn convert(path: &PathBuf) -> Result<Config, SwayconfError> {
    log::info!("Opening file {}", path.display());
    let str = fs::read_to_string(path)?;
    log::info!("Parsing configuration: {}", path.display());
    let cfg: Config = toml::from_str(&str)?;
    log::debug!("Everything went okay, continuing");
    Ok(cfg)
}

fn write(path: &PathBuf, cfg: Config) -> Result<usize, IoError> {
    log::info!("Writing to file {}", path.display());
    let mut file = File::create(path.clone())?;
    file.write(cfg.to_string().as_bytes())
}

fn reload_sway() {
    log::info!("Attempting to reload config via swaymsg...");
    match Command::new("swaymsg").arg("reload").output() {
        Ok(output) => {
            log::debug!("{}", std::str::from_utf8(&*output.stdout).unwrap_or("Error reading stdout"));
            if output.status.success() {
                log::debug!("{}", std::str::from_utf8(&*output.stderr).unwrap_or("Error reading stderr"));
                log::info!("swaymsg exited with status {}", output.status);
            } else {
                log::error!("swaymsg exited with status {}", output.status);
                log::error!("Output of stderr:");
                log::error!("{}", std::str::from_utf8(&*output.stderr).unwrap_or("Error reading stderr"));
            }
        }
        Err(err) => {log::error!("swaymsg call failed: {}", err);}
    }
}

/// Main entrypoint
fn main() {
    env_logger::init();

    let args = Args::parse();

    let path = args.input_file.path().to_path_buf();
    match convert(&path) {
        Ok(cfg) => {
            log::info!("Successfully converted {}", &path.display());
            log::trace!("{:#?}", &cfg);
            let write_path = match args.output_file {
                Some(p) => p.path().to_path_buf(),
                None => path.with_extension("")
            };
            match write(&write_path, cfg) {
                Ok(_) => {
                    log::info!("Successfully wrote to {}", &write_path.display());
                    if args.reload { reload_sway() }
                },
                Err(e) => log::error!("Failed to write to {}: {}", &write_path.display(), e),
            }
        }
        Err(err) => {
            log::error!("Failed to convert {}: {}", &path.display(), err);
        }
    };
}
