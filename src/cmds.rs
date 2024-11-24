// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    utils::{is_root, which},
    Cmd,
};

pub const PACMAN_BIN: &str = "pacman";
pub const PARU_BIN: &str = "paru";
const SUDO_BIN: &str = "sudo";

pub fn pacman() -> Cmd {
    Cmd::new(PACMAN_BIN)
}

pub fn sudo_pacman() -> Cmd {
    if is_root() {
        return pacman();
    }
    sudo().arg(PACMAN_BIN)
}

pub fn paru_if_present() -> Option<Cmd> {
    which(PARU_BIN)
}

pub fn paru_or_sudo_pacman() -> Cmd {
    paru_if_present().unwrap_or_else(sudo_pacman)
}

pub fn paru_or_pacman() -> Cmd {
    paru_if_present().unwrap_or_else(pacman)
}

fn sudo() -> Cmd {
    Cmd::new(SUDO_BIN)
}
