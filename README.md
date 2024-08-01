# lifecycler

[_bevy game jam #5 submission_](https://itch.io/jam/bevy-jam-5)

![logo](https://assets.cxreiff.com/github/lifecycler.png)

An aquarium that runs in your terminal!

A decoration/fidget-toy that lets you watch your fishes' lifecycle while you code. Test drive of my plugin [bevy_ratatui_render](https://github.com/cxreiff/bevy_ratatui_render), a plugin that lets you render a bevy application to the terminal using [ratatui](https://github.com/ratatui-org/ratatui)/[ratatui-image](https://github.com/benjajaja/ratatui-image).

## play with cargo

If you have [cargo](https://github.com/rust-lang/cargo) installed, you can simply run the following:

```sh
cargo install lifecycler --locked
```

...and then to launch the game:

```sh
lifecycler
```

## controls

|                    |                            |
|--------------------|----------------------------|
| Left Click or Drag | Dispense a food pellet.    |
| Space Bar          | Toggle day/night modes.    |
| M                  | Mute/unmute sound effects. |
| Q                  | Quit the game.             |

## resolution

The resolution is determined by the character-wise dimensions of your terminal- so zoom out in your terminal for more detail, zoom in for a more pixelated look.

## other install methods

### distro packages

#### AUR

Arch Linux users can also install [from the AUR](https://aur.archlinux.org/packages/lifecycler) using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers):

```sh
paru -S lifecycler
```

### manually

Alternatively you can manually download an executable from the [itch.io page](https://cxreiff.itch.io/lifecycler) or [github releases](https://github.com/cxreiff/lifecycler/releases).

If you manually installed the executable, you will have to include the path to launch it (e.g. `./lifecycler` if in the same directory).

On macOS you may need to Right-Click > Open With, selecting a supported terminal, and then selecting Open in order to bypass code signing (I haven't figured out code signing yet).

## issues

I am still hunting down platform-specific problems with rendering and input- if you run into one, please open an issue!

## compatibility

This requires that your terminal:

1. Supports 24bit color.
2. Has reasonably efficient rendering.

This includes a decent variety of terminals, but I have personally confirmed good results in the following:

- Alacritty
- Kitty
- WezTerm
- iTerm2

## gifs

![day mode](https://assets.cxreiff.com/github/lifecycler_day.gif)![night mode](https://assets.cxreiff.com/github/lifecycler_night.gif)
