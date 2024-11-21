//! Reused pacman commands

use crate::{cmds::pacman, command::Cmd};

pub fn files_of_installed_pkgs() -> Cmd {
    pacman().arg("-Ql")
}

pub fn installed_packages() -> Cmd {
    pacman().arg("-Qq")
}
