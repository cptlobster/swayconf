# swayconf
Configuration generator for the [Sway window manager](https://github.com/swaywm/sway).

*Note: This project is still under heavy development and isn't even in alpha yet. Once I am able to generate my personal
Sway config (see samples/config.toml), I may consider it an alpha build. Some of the features listed are not implemented
yet, and EVERYTHING is subject to change.*

## Features

- Write your Sway configuration using TOML
  - Support for some Sway commands (WIP)
  - Automatically generate common groups of commands using our autogen module (coming soon)
  - Avoid errors in your Sway configuration with built-in validation (coming soon)
  - Export your config directly to Sway's command format

### Goals

- Full support for Sway config
  - Includes sway-output, sway-input, and swaybar.
- Support for additional programs
  - Handle differences in configuration for i3
  - Configuration for Sway extension programs (swaylock, swayidle, swaybg)
  - Common third-party programs used with Sway (i.e. waybar, rofi, dmenu, mako, dunst, etc.)
    - Possibly some sort of schema format to automatically generate config structure, or make it module based
  - (very long way off) Other tiling window managers (i.e. hyprland, xmonad)
- Graphical interface for managing configuration files
- More examples / documentation

## Getting Started

First, clone the repo:
```shell
git clone https://github.com/cptlobster/swayconf.git
cd swayconf
```

The project is managed using Cargo. Make sure that it is installed, then you can build and run using:
```shell
cargo build
cargo run
```

## Documentation

Documentation is a work in progress. Rustdoc can be generated using `cargo doc`, but a proper user guide does not exist.
Potentially could resolve this by setting up Sphinx alongside this project and generating documentation there (there's
an extension for Rustdoc support).

## License
This program is licensed under the [GNU General Public License, version 3](LICENSE.md).

*This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public
License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later
version.*<br />
*This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.*
<br />
*You should have received a copy of the GNU General Public License along with this program. If not, see
https://www.gnu.org/licenses/.*