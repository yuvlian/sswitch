# sswitch

peak CLI steam account switcher on god.

mostly made this for myself cuz i got a bunch of CS prime redeem keys and switching with the steam's default switcher is... annoying.

## features

- written in rust with 0 external dependency

- optimized (i think)

- crossplatform (uses OS specific implementations selected at compile time).
  - if ur actually on linux/macos please feel free to report bugs

- can give tags to accounts and just use the tag to launch them

- disables `"Who's playing?"` stuff automatically i guess

- autodetect steam path

uhh idk what else to type js read the command reference dawg (or source code uwu).

## cli reference

| command| explanation |
| :--- | :--- |
| `sswitch` or `sswitch start` | prints steam path, status, active accounts, and available accounts. then launch depending on selection |
| `sswitch <tag>` | closes steam, switch & launch account with said tag |
| `sswitch stop` | stops steam & steamwebhelper |
| `sswitch path <path>` | configure custom steam path (it will auto detect by default) |
| `sswitch skip-offline` | for all users in `loginusers.vdf` set the `SkipOfflineModeWarning` to `1` |
| `sswitch tag <tag> <acc>` | assign a tag to an account so u can launch them faster. `<acc>` can be the account index (1, 2, etc.), username, display name, or Steam ID. |
| `sswitch untag <tag>` | remove tag |
| `sswitch tags` | lists all tags and associated account |
| `sswitch help` | self explanatory |

## setup (from source)

1. install rust (duh)

2. build

   ```
   cargo install --git https://github.com/yuvlian/sswitch --bin sswitch
   ```

   you can also do:

   ```
   git clone https://github.com/yuvlian/sswitch.git
   cd sswitch
   cargo install --path sswitch
   ```

NOTE: `cargo install` automatically builds with release profile & adds bin to `.cargo/bin` which is in PATH env

## setup (prebuilt)

open releases page. download prebuilt, extract, and add to PATH env var.

NOTE: im sharing windows build only, crosscompiling rust is annoying.
