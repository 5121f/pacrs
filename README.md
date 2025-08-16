# pacrs

**pacrs** - `pacman` and `paru` wrapper (yes, wrapper on top of another
wrapper), with friendly CLI inspired by zypper and apt.

paru is optional dependency needed for AUR support.

Some features and peculiarities:

- Semi-automatic index update. It's good because running `pacman -Sy`
  without immediately upgrading is potentially dangerous (see
  [Arch Wiki](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported)
  for more info).
- If you try to install a package that has been updated in the repository,
  `pacrs` will inform you that you need to update your system first.
- `pacrs autoremove` works like `pacman -Rs` or `apt autoremove`. If no
  package is specified, it  behaves like `paru -c`.
- `pacrs ps` command which works like `zypper ps`.
- `pacman -Si` and `pacman -Qi` were merged into `pacrs info` command.
- `pacman -F` and `pacman -Ql` were merged into `pacrs files` command.
- `pacrs list-updates` is implementation of `checkupdates` script from
  `pacmancontrib` package. It works as if you updated the index and ran
  `pacman -Qu`, but `pacrs` does not affect your local index.
- `pacrs packages` supports multiple filters, which allow you to find, for
  example, AUR packages installed as dependencies -
  `pacrs packages --aur --deps`.

for more info see `pacrs help` command output.


## Pros and cons in relation to `pacman` and `paru`

- `pacrs` has more intuitive and consistent interface.
- `pacrs` removes some manual work.
  - This makes it more user-friendly.
  - This makes it slower in some cases.
- `pacrs` does not set a goal to be a complete replacement for `pacman`
  and `paru` and does not provide some advanced features they have. For
  cases where you need these features you can use `pacman` or `paru`
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

To build from source you need
[rust toolchain](https://www.rust-lang.org/tools/install). For
installation, you need [just](https://github.com/casey/just).

```
git clone git@github.com:5121f/pacrs.git
cd pacrs
cargo build --release
just install
```

for uninstall, use `just uninstall`.

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
