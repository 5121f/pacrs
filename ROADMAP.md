- Uses pacman directly if paru is not installed.
- `pacrs install --reinstall` command which should check cache and if the
  packages are contained in it, reinstall these packages without check.
- `pacrs list --files` - alias of `pacman -F`.
  - How we should process index?
    - Update by default. If error - skip.
- `pacrs list --aur` alias for `pacman -Qm`
- Opportunity to use multiple filters for `list` command. Example:
  `pacrs list --upgradable --orphaned`
- Clean cache functionality (`pacman -Sc`, `paccache`)
  - Configuration for automaticly clean.
