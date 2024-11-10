use crate::{
    temp_db::{initialize_temp_db, TEMP_DB_PATH},
    Cmd, PacrsAlpm, TempAlpm,
};

use anyhow::{anyhow, bail};

pub const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";

fn program_is_present() -> anyhow::Result<bool> {
    Ok(Cmd::new("which").ignore_error()?.code().unwrap() == 0)
}

fn paru_or_pacman() -> anyhow::Result<Cmd> {
    let cmd = program_is_present()?
        .then(|| Cmd::new(PARU_BIN))
        .unwrap_or_else(|| Cmd::new(PACMAN_BIN));
    Ok(cmd)
}

pub fn list() -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Qq").execute()?;
    Ok(())
}

pub fn info(package: String) -> anyhow::Result<()> {
    const COMMAND: &str = PACMAN_BIN;
    let exit_status = Cmd::new(COMMAND).args(["-Qi", &package]).ignore_error()?;
    let exit_code = exit_status
        .code()
        .ok_or_else(|| anyhow!("Failed to execute {COMMAND}"))?;
    if exit_code == 0 {
        return Ok(());
    }
    Cmd::new(COMMAND).args(["-Si", &package]).execute()?;
    Ok(())
}

pub fn search(package: String) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).args(["-Ss", &package]).execute()?;
    Ok(())
}

pub fn install(packages: Vec<String>) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    let alpm_tmp = TempAlpm::new()?;
    if alpm.pkgs_or_their_deps_was_updated_in_db(&alpm_tmp, packages.clone())? {
        bail!(
            "One or more package you will want to install or their dependencies was updated in \
            the repo. Upgrade your system with 'pacrs upgrade' befor install it."
        );
    }
    paru_or_pacman()?.arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn remove(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Rs").args(packages).execute()?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    paru_or_pacman()?.arg("-Syu").args(packages).execute()?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    initialize_temp_db()?;
    Cmd::new(PACMAN_BIN)
        .args(["-Qu", "--dbpath", TEMP_DB_PATH])
        .execute()?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Qdtq").execute()?;
    Ok(())
}

pub fn remvoe_orphaned_packages() -> anyhow::Result<()> {
    let orphaned_packages = Cmd::new(PACMAN_BIN)
        .arg("-Qdtq")
        .execute_and_grub_output()?
        .split("\n")
        .map(|line| line.to_owned())
        .collect();
    remove(orphaned_packages)?;
    Ok(())
}

pub fn mark_explicit(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN)
        .args(["-D", "--asexplicit"])
        .args(packages)
        .execute()?;
    Ok(())
}

pub fn mark_dep(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN)
        .args(["-D", "--asdeps"])
        .args(packages)
        .execute()?;
    Ok(())
}
