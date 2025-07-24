# SPDX-License-Identifier: GPL-3.0-only

progname := 'pacrs'
target_bin := '/usr/bin' / progname
target_fish_completion :=  '/usr/share/fish/vendor_completions.d' / progname + '.fish'
target_bash := '/usr/share/bash-completion/completions' / progname + '.bash'
target_zsh := '/usr/share/zsh/site-functions/_' + progname

install:
    install -Dm0755 target/release/{{progname}} {{target_bin}}
    install -Dm0644 completions/fish {{target_fish_completion}}
    install -Dm0644 completions/bash {{target_bash}}
    install -Dm0644 completions/zsh {{target_zsh}}

uninstall:
    rm {{target_bin}}
    rm {{target_fish_completion}}
    rm {{target_bash}}
    rm {{target_zsh}}
