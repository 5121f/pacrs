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
- `pacrs ps` - command which should print list of running processes on the
  system which continue to use meanwhile deleted files. `zypper ps`
  alternative. [pacman-pstatus](https://gitlab.com/renyuneyun/pacman-ps)
  package from AUR realizes something like that.
