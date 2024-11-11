use crate::Cmd;

pub const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";
const WHICH_BIN: &str = "which";
const SUDO_BIN: &str = "sudo";

pub fn pacman() -> Cmd {
    Cmd::new(PACMAN_BIN)
}

pub fn sudo_pacman() -> Cmd {
    sudo().arg(PACMAN_BIN)
}

pub fn sudo_paru_or_pacman() -> anyhow::Result<Cmd> {
    let cmd = program_is_present()?.then(paru).unwrap_or_else(sudo_pacman);
    Ok(cmd)
}

pub fn paru_or_pacman() -> anyhow::Result<Cmd> {
    Ok(program_is_present()?.then(paru).unwrap_or_else(pacman))
}

fn paru() -> Cmd {
    Cmd::new(PARU_BIN)
}

fn program_is_present() -> anyhow::Result<bool> {
    Ok(which().arg(PARU_BIN).execute(false)?.success())
}

fn sudo() -> Cmd {
    Cmd::new(SUDO_BIN)
}

fn which() -> Cmd {
    Cmd::new(WHICH_BIN)
}
