use crate::Cmd;

pub const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";

pub fn pacman() -> Cmd {
    Cmd::new(PACMAN_BIN)
}

pub fn paru_or_pacman() -> anyhow::Result<Cmd> {
    let cmd = program_is_present()?
        .then(|| Cmd::new(PARU_BIN))
        .unwrap_or_else(|| Cmd::new("sudo").arg(PACMAN_BIN));
    Ok(cmd)
}

fn program_is_present() -> anyhow::Result<bool> {
    Ok(Cmd::new("which")
        .arg(PACMAN_BIN)
        .hide_output_and_give_exit_status()?
        .success())
}
