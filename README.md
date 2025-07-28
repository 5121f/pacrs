# pacrs

**pacrs** - `pacman` and `paru` wrapper (yes, wrapper above wrapper), with
frendly CLI inspired by zypper and apt.

paru is optional dependencie needed for AUR support.

Some features and peculiarities:

- Semi-automatic index update. It's good because `pacman -Sy` command ran
  without further update is potentially fearing (see
  [Arch Wiki](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported)
  for more info).
- If you try to install package which updated in the repo pacrs inform you
  what you needed to update system before.
- `pacrs autoremove` command which works like `pacman -Rs` or
  `apt autoremove` or if no package given it works like `paru -c`.
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
- `pacrs` does not set a goal to be a complete replacement for `pacman`
  and `paru` and not provide some advanced features which they does. For
  cases when you need this features you can use `pacman` or `paru`
  directly (or open the issue with feature request).

## Installation

### With `paru`

```
paru -S pacrs
```

## From AUR

```
git clone https://aur.archlinux.org/pacrs.git
cd pacrs
makepkg -si
```

## From source

For build from source you need
[rust toolchain](https://www.rust-lang.org/tools/install). For
installation you need [just](https://github.com/casey/just).

```
git clone git@github.com:5121f/pacrs.git
cd pacrs
cargo build --release
just install
```

for uninstall use `just uninstall`.

## License

This program is free software.
It is licensed under the GNU GPL version 3.
That means you are free to use this program for any purpose;
free to study and modify this program to suit your needs;
and free to share this program or your modifications with anyone.
If you share this program or your modifications
you must grant the recipients the same freedoms.
To be more specific: you must share the source code under the same license.
For details see https://www.gnu.org/licenses/gpl-3.0.html
