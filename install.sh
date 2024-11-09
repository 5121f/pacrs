#!/usr/bin/env sh
progname=pacrs

install -Dm0755 target/release/$progname /usr/bin/$progname
install -Dm0644 completions/fish /usr/share/fish/completions/$progname.fish
