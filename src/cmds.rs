use crate::Cmd;

pub const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";

pub fn pacman() -> Cmd {
    Cmd::new(PACMAN_BIN)
}

pub fn sudo_pacman() -> Cmd {
    sudo().arg(PACMAN_BIN)
}

pub fn sudo_paru_or_pacman() -> anyhow::Result<Cmd> {
    let cmd = program_is_present()?
        .then(|| Cmd::new(PARU_BIN))
        .unwrap_or_else(|| sudo().arg(PACMAN_BIN));
    Ok(cmd)
}

fn program_is_present() -> anyhow::Result<bool> {
    Ok(which()
        .arg(PACMAN_BIN)
        .hide_output_and_give_exit_status()?
        .success())
}

fn sudo() -> Cmd {
    const SUDO_BIN: &str = "sudo";
    Cmd::new(SUDO_BIN)
}

fn which() -> Cmd {
    const WHICH_BIN: &str = "which";
    Cmd::new(WHICH_BIN)
}
