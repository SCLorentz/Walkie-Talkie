# Linux

On linux there are some packages that you will need to compile this project. Make sure you have installed `clang`, `build-essential`, `lld`, `libvulkan-dev`, `vulkan-tools` and `vulkan-validation-layers`. To build for linux simply use `cargo build-linux-x64`. By default the builds are for wayland.

## Wayland

Different from windows or macOS, the linux implementation has a lot of checks in runtime. By default it would be preferable to use server side decorations everywhere, but since some DEs like Gnome don't offer support for the XDG_DECORATION wayland protocol, checking the necessity to render a CSD it's not an option.

The feature of "CSD" will be avaliable as an extra dependency avaliable as default, but, if a user wants to compile for themselves without it, the option will be avaliable.

## X11

todo

### things that should be controlled by the server:

- Text and font;
- Decorations, shadows and control buttons;
- Popups;
- Notifications;
- Context menu;
- App menu, [cosmic feature request](https://github.com/pop-os/cosmic-epoch/issues/894);

### Wayland Protocols I want to support on 'Linux'

https://crates.io/crates/wayland-protocols-wlr

by design, supported DE's:

- KDE (primary focus)
- GNOME
- COSMIC
- SWAY
- HYPRLAND

| source                                              | name                    | DE support             |
|-----------------------------------------------------|-------------------------|------------------------|
| https://wayland.app/protocols/xdg-dialog-v1         | xdg_dialog              | KDE / Hyprland / GNOME |
| https://wayland.app/protocols/linux-drm-syncobj-v1  | linux-drm-syncobj       | All                    |
| https://wayland.app/protocols/cursor-shape-v1       | cursor-shape            | All                    |
| https://wayland.app/protocols/wayland-protocols/461 | xdg_decoration_theme    | Not implemented        |
| https://wayland.app/protocols/wayland-protocols/449 | xdg_surface_shape       | Not implemented        |
| None                                                | xdg_chrome_capabilities | Not implemented        |

### xdg_chrome_capabilities

Idea similar to cocoa functionallity with the Quartz compositor on apple, but the implementation doesn't relly on the client side decoration like on apple, insted it asks _politely_ to the server to handle that.

```Rust
window.setTitlebarAppearsTransparent(true);
window.setTitleVisibility(NSWindowTitleVisibility(1));
```
code from './crates/app/src/platform/cocoa.rs'

The best of both CSD and SSD, no anoying title bar using vertical space and native decorations.
Currentlly on wlroots local test.

<img src="./doc/sources/xdg_chrome_capabilities.jpg" alt="xdg_chrome_capabilities examplification" width="600">
