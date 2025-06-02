# pacrs changelog

## Unreleased

- changled alias for `autoremove` from `arm` to `ar`
- `clean`: request root password if needed
- `packages`: added `search` key for search in installed packages
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
