# Galactic

Beautifully minimal terminal emulator.

## About

You might question the necessity of yet another GTK terminal emulator, given the
plethora of existing options. However, many of these alternatives are
feature-rich and rely heavily on complex graphical configuration. Galactic, on
the other hand, stands out by embracing simplicity. It adopts a traditional
approach, utilizing a straightforward configuration file and avoiding additional
graphical features apart from the terminal itself.

## Configuration

Galactic is configured using files in the TOML format. The main configuration
file is either located at `$XDG_CONFIG_HOME/galactic/config.toml` or
`$HOME/.config/galactic/config.toml`. Every configuration entry is optional.

### General

The configuration under the `[general]` section.

#### Command

```
command = "<string>" | ["<string>"]
```

The command that will be run in the terminal. Defaults to the `$SHELL`
environment variable or `/usr/bin/bash` if it is note set.

#### Font

```
font = "<string>"
```

The font description of the font used in the terminal. Defaults to
`"Monospace 12"`.

#### Theme

```
theme = "<string>"
```

The theme to be used for the terminal. See section [Themes](#themes) for more
information. Defaults to using the _Adwaita Dark_ theme.

### Window

The configuration under the `[window]` section.

#### Title

```
title = "<string>"
```

The title that will be displayed in the header bar. Defaults to `"Galactic"`.

#### Scroll Bar

```
scroll_bar = true | false
```

Whether to show a vertical scroll bar or not. Defaults to `true`.

#### Size

```
size = { width = <integer>, height = <integer> }
```

The default size of the window in pixels. Defaults to
`{ width = 1200, height = 800 }`.

#### Padding

```
padding = { x = <integer>, y = <integer> }
```

The default padding inside the window in pixels. Defaults to `{ x = 8, y = 8 }`.

## Themes

Themes can be placed in the `themes` directory inside of the configuration
directory. Theme files are stored in the TOML format. The theme configuration
setting needs to be set to the theme file name without any extension.

### Format

The theme file consists of a foreground and background color, as well as a color
palette. They correspond to theme files from other GTK/VTE based terminal
emulators, like GNOME Terminal or Tilix.

An example theme file of the _Adwaita Dark_ theme:

```toml
foreground = "#ffffff"
background = "#1e1e1e"
palette = [
  "#241F31",
  "#C01C28",
  "#2EC27E",
  "#F5C211",
  "#1E78E4",
  "#9841BB",
  "#0AB9DC",
  "#C0BFBC",
  "#5E5C64",
  "#ED333B",
  "#57E389",
  "#F8E45C",
  "#51A1FF",
  "#C061CB",
  "#4FD2FD",
  "#F6F5F4",
]
```

## Options

Command line options can be used to alter the behavior of the application.

| Option             | Description                           |
| ------------------ | ------------------------------------- |
| `--default-config` | Use the default configuration         |
| `--config-dir`     | Overwrite the configuration directory |

## Contributing

Contributions in any way are very welcome. However, please stick to existing
styles of patterns and formatting.

### Development

You need to have the latest stable version of Rust installed. GTK build
dependencies need to be installed seperately:

```sh
# Use DNF on Fedora
dnf install gcc blueprint-compiler gtk4-devel libadwaita-devel vte291-gtk4-devel
```

```sh
# Use Pacman on Arch
pacman -S base-devel blueprint-compiler gtk4 libadwaita vte4
```

## License

See [License](license.md) for more information.
