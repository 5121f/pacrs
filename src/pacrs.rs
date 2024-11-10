use crate::{
    pacman::{pacman, paru_or_pacman},
    temp_db::{initialize_temp_db, TEMP_DB_PATH},
    PacrsAlpm, TempAlpm,
};

use anyhow::bail;

pub fn list() -> anyhow::Result<()> {
    pacman().arg("-Qq").execute()?;
    Ok(())
}

pub fn info(package: String) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    let is_local_pkg = alpm.localdb().pkg(package.as_str()).is_ok();
    if is_local_pkg {
        pacman().args(["-Qi", &package]).execute()?;
        return Ok(());
    }
    pacman().args(["-Si", &package]).execute()?;
    Ok(())
}

pub fn search(package: String) -> anyhow::Result<()> {
    pacman().args(["-Ss", &package]).execute()?;
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
    pacman().arg("-Rs").args(packages).execute()?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    paru_or_pacman()?.arg("-Syu").args(packages).execute()?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    initialize_temp_db()?;
    pacman().args(["-Qu", "--dbpath", TEMP_DB_PATH]).execute()?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<()> {
    pacman().arg("-Qdtq").execute()?;
    Ok(())
}

pub fn remvoe_orphaned_packages() -> anyhow::Result<()> {
    let orphaned_packages = pacman()
        .arg("-Qdtq")
        .execute_and_grub_output()?
        .split("\n")
        .map(|line| line.to_owned())
        .collect();
    remove(orphaned_packages)?;
    Ok(())
}

pub fn mark_explicit(packages: Vec<String>) -> anyhow::Result<()> {
    pacman()
        .args(["-D", "--asexplicit"])
        .args(packages)
        .execute()?;
    Ok(())
}

pub fn mark_dep(packages: Vec<String>) -> anyhow::Result<()> {
    pacman().args(["-D", "--asdeps"]).args(packages).execute()?;
    Ok(())
}
