# Roadmap of `pacrs`

- `pacrs install --from-cache` command which should check cache and if the
  packages are contained in it, reinstall these packages without check.
- Extended clean cache functionality (`paccache -rk2`)
  - Configuration for automaticly clean.
- `pacrs repo` command.
  - `--list` print list of repos.
  - `--clean` remove unused repos (like it `pacman -Sc` does).
  - `--add` add repo in `pacmanconf`?
  - `--remove` remove repo from `pacmanconf`?
