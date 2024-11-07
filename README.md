# pacrs

## Differences in the logic with Pacman

- No need to manage index. pacrs do it under the hood.
  - No more risks to forget you performed `pacman -Sy` and installed package after
    that and which caused dependency issues.
- If you try to try to install package which updated in the repo pacrs inform you]
  (without updating the main index) what you needed to upgrade system.
- `pacman -Si` and `pacman -Qi` merged in single `pacrs info` command which by
  default search into internet and with an error search into local index.
