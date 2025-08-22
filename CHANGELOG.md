# pacrs changelog

## Unreleased

### Breaking changes

- rename subcommand `list-updates` to `listupdates`
- `autoremove`: changled short alias from `arm` to `ar`

### Other changes

- `clean`
  - added `keep` flag for preserve some packets in cache instead of
    deleting them all
  - added `show-remove-candidates` which now dose't work with
    `uninstalled` flag
  - request root password if needed (not with `keep` flag)
- `info`: added `recursive-deps` flag
- `autoremove`: added opportunity to pass packages
- `install`: fix outdated check
- `packages`: added `search` key for search in installed packages
- `files`: don't print directories
- added logging with `RUST_LOG` environment variable (how to use it see
  [here](https://docs.rs/env_logger/latest/env_logger/#enabling-logging))
- added basic completions for `bash` and `zsh`
- actualized fish completions

## 0.3.0 - 2024-12-01

### Breaking changes

- changed some aliases
  - `i` for `info` to `if` (like in zypper)
  - `f` for `files` to `fl`
  - `m` for `mark` to `mr`

### Other changes

- added `reverse` key for ps command
- fix typos

## v0.2.0 - 2024-11-23

- `ps` command changes
  - fix incorrect data output
  - added `sort-by` key
  - added info message if running without root privileges
    - added `quiet` key for hide it
- `pacrs install` Fix wrong command hint

## v0.1.0 - 2024-11-22

First public release.
