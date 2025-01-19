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
/// Runtime command enumeration.
/// 
/// This module should ONLY contain the enum for runtime commands, all options should be handled in
/// the [options] module.
pub mod runtime;
/// Command options and arguments.
/// 
/// The module itself contains some common arguments (used by various config and runtime commands),
/// as well as modules for specific commands that have multiple variants / hyper specific arguments.
pub mod options;
/// Base config file generation.
///
/// This has a rigid structure for config-only commands, so that [serde] can assemble/disassemble
/// TOML in a way that is even moderately comprehensible.
pub mod config;