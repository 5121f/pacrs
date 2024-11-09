# pacrs

## Differences in the logic with Pacman

- No need to manage index. pacrs do it under the hood.
  - No more risks to forget you performed `pacman -Sy` and installed package
    after that and which caused dependency issues (for the reasons why it's
    bad, see
    [Arch Wiki](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported).
- If you try to try to install package which updated in the repo pacrs inform
  you (without updating the main index) what you needed to upgrade system.
- `pacman -Si` and `pacman -Qi` merged in single `pacrs info` command which by
  default search into local index and with an error search into internet.
- `pacrs list --upgradable` is alternative to `pacman -Qu` but uses actual info
  as if you would update indexes with `pacman -Sy` but pacrs actually not
  affects your local index for that.
