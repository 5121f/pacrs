# pacrs

## Differences in the logic with `pacman`

- No need to manage index. pacrs do it under the hood.
  - No more risks to forget you performed `pacman -Sy` and installed package
    after that and which caused dependency issues (for the reasons why it's
    bad, see
    [Arch Wiki](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported).
- If you try to install package which updated in the repo pacrs inform you
  (without updating the main index) what you needed to upgrade system.
- `pacman -Si` and `pacman -Qi` merged in single `pacrs info` command which by
  default search into local index and with an error search into internet.
- `pacrs list --upgradable` is alternative to `pacman -Qu` but uses actual info
  as if you would update indexes with `pacman -Sy` but pacrs actually not
  affects your local index for that.
- `pacrs packages` supports multiple filters which allows you to find, for
  example AUR packages installed as dependencie -
  `pacrs packages --aur --deps`.

## Pros and cons in relation to `pacman` and `paru`

- `pacrs` has more intuitive and consistent interface.
- `pacrs` conducts some checks and removes part of handmade.
  - This makes him more user-friendly.
  - This makes him much slower in some cases.
- `pacrs` does not set a goal to be a complete replacement for `pacman` and
  `paru` and not provide some advanced features which they does. For cases
  when you need this features you can use `pacman` or `paru` directly (or open
  the issue with feature request).
