# Roadmap of `pacrs`

- `pacrs ps --sort-by <value> --reverse` and `pacrs ps --shorter --reverse`
- `pacrs install --from-cache` command which should check cache and if the
  packages are contained in it, reinstall these packages without check.
- Extended clean cache functionality.
  - show cleaned size.
  - `--keep <n>` key for save last 'n' package caches.
  - Configuration for automatically clean.
