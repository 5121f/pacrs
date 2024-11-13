use crate::{utils::program_is_present, Cmd};

pub const PACMAN_BIN: &str = "pacman";
pub const PARU_BIN: &str = "paru";
const SUDO_BIN: &str = "sudo";

pub fn pacman() -> Cmd {
    Cmd::new(PACMAN_BIN)
}

pub fn sudo_pacman() -> Cmd {
    sudo().arg(PACMAN_BIN)
}

pub fn paru_or_sudo_pacman() -> Cmd {
    program_is_present(PARU_BIN)
        .then(paru)
        .unwrap_or_else(sudo_pacman)
}

pub fn paru_or_pacman() -> Cmd {
    program_is_present(PARU_BIN)
        .then(paru)
        .unwrap_or_else(pacman)
}

fn paru() -> Cmd {
    Cmd::new(PARU_BIN)
}

fn sudo() -> Cmd {
    Cmd::new(SUDO_BIN)
}
