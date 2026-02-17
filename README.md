# Tealinux Modularitea

Tealinux modularitea is a set of GUI apps that mimic how microsoft store and KDE utility behave, this allow you to install apps easily, tune your linux as you needs, and help optimize something in your system

# building

```sh
git clone git@github.com:tealinuxos/tealinux-modularity.git && \
cd tealinux-modularity && bun install && bunx tauri dev
```

note that, you need this dependencies as its need to function properly

- pkexec (rootkit utility)
- GTK
- bun (for building)

also, please set your ENV to `TEALINUX_BUILD=dev` or `TEALINUX_BUILD=prod` when building, this allow you to avoid clicking dangerous parts of this software (such changing grub theme, disabling swap, etc). Another option is `TEALINUX_BUILD_SHOW_SPLASH=boolean`, allow you to force show splash screen by overriding `$HOME/.config/tealinux-modularity` file 

# License 
MIT