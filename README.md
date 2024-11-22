# pacrs

**pacrs** - `pacman` and `paru` wrapper (yes, wrapper above wrapper), with
frendly CLI inspired by zypper and apt.

paru is optional dependencie needed for AUR support.

Some features and peculiarities:

- Semi-automatic index update. It's good because `pacman -Sy` command runned
  without further update is potentially fearing (see
    [Arch Wiki](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported)
    for more info).
- If you try to install package which updated in the repo pacrs inform you
  what you needed to update system before.
- `apt autoremove` command which works like `apt autoremove` or `paru -c`.
- `pacrs ps` command which works like `zypper ps`.
- `pacman -Si` and `pacman -Qi` was merged into `pacrs info` command.
- `pacman -F` and `pacman -Ql` was merged into `pacrs files` command.
- `pacrs list-updates` is implementation of `checkupdates` script from
  `pacmancontrib` package. It works like if you update index and run
  `pacman -Qu` but pacrs bot affects your local index for that.
- `pacrs packages` supports multiple filters which allows you to find, for
  example AUR packages installed as dependencie -
  `pacrs packages --aur --deps`.

for more info see `pacrs help` command output.


## Pros and cons in relation to `pacman` and `paru`

- `pacrs` has more intuitive and consistent interface.
- `pacrs` conducts some checks and removes part of handmade.
  - This makes him more user-friendly.
  - This makes him slower in some cases.
- `pacrs` does not set a goal to be a complete replacement for `pacman` and
  `paru` and not provide some advanced features which they does. For cases
  when you need this features you can use `pacman` or `paru` directly (or open
  the issue with feature request).
