- `pacrs install --reinstall` command which should check cache and if the
  packages are contained in it, reinstall these packages without check.
- `pacrs packages --files` - alias of `pacman -F`.
  - How we should process index?
    - Update by default. If error - skip.
- Opportunity to use multiple filters for `packages` command. Example:
  `pacrs packages --upgradable --orphaned`
- Clean cache functionality (`pacman -Sc`, `paccache`)
  - Configuration for automaticly clean.
- `pacrs repo` command.
  - `--list` print list of repos.
  - `--clean` remove unused repos (like it `pacman -Sc` does).
  - `--add` add repo in `pacmanconf`?
  - `--remove` remove repo from `pacmanconf`?
