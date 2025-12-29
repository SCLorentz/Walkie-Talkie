# Walkie Talkie

My own implementation of the Client-Server API on a fast and optimized client program.

Chatting apps like whatsapp, discord, snapchat, messager, telegram, etc are upsetting. All those apps are heavy, specially on computers, since most of those are made in electron or not optimized. Whatsapp on my macOS occupies 487MB of space; They are bloated with AI features like meta AI and trackers to sell your data to other companies; This apps are simply not optimized to be run with high resource usage computers, like in gaming, specially for low end computers; And most importantly, they are closed source.

## Development

None of the options for creating user interfaces for multiplatform pleased me, so I'm doing one from scratch using the Vulkan rendering API and the native calls from macOS, android, windows and linux. The main objective is to create something minimalistic portable and native in all platforms and desktops. I want to be able to use those crates on other projects as well and in the future, move them to their own repo.

### Linux

Different from windows, macOS and android, the linux implementation has a lot of checks in runtime. By default it would be preferible to use server side decorations everywhere, but sice some DEs like Gnome don't offer support for the XDG_DECORATION wayland protocol, checking the necessity to render a CSD it's not an option. For now I don't bother to create a CSD, but the option is avaliable and maybe someday I will implement it.

### Vulkan

To create more compatibility between different systems (mainly macOS), vulkan will not be statically linked to the program, instead it will be loaded at runtime.

## Build

The main crates to build the UI are avaliable on `./crates/app`, `./crates/renderer` and `./crates/gui`. To build yourself the app from the source you just need to use `cargo build --release`, but for now there's no packaging avaliable for any platform.

### MacOS

On MacOS you will need to install the vulkan SDK to be able to build the program

### Android

To build for android, make sure that the android sdk and ndk are installed and configured on your machine. You wont be able to compile the program without them. Then you can just use `cargo android-build`.
