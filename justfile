progname := 'pacrs'
target_bin := '/usr/bin' / progname
target_fish_completion :=  '/usr/share/fish/completions' / progname + '.fish'

install:
    install -Dm0755 target/release/{{progname}} {{target_bin}}
    install -Dm0644 completions/fish {{target_fish_completion}}

uninstall:
    rm {{target_bin}}
    rm {{target_fish_completion}}
