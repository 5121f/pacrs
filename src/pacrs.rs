use std::{os::unix::fs::MetadataExt, path::Path};

use crate::{
    alpm::pacmanconf,
    cmds::{pacman, paru_or_pacman, sudo_pacman, sudo_paru_or_pacman},
    temp_db::{initialize_temp_db, TempAlpm, TEMP_DB_PATH},
    PacrsAlpm,
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
    paru_or_pacman()?.args(["-Si", &package]).execute()?;
    Ok(())
}

pub fn search(package: String) -> anyhow::Result<()> {
    paru_or_pacman()?.args(["-Ss", &package]).execute()?;
    Ok(())
}

pub fn cache_clean() -> anyhow::Result<()> {
    pacman().arg("-Scc").execute()?;
    Ok(())
}

pub fn cache_clean_uninstalled() -> anyhow::Result<()> {
    pacman().arg("-Sc").execute()?;
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
    sudo_paru_or_pacman()?.arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn list_aur() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qmq").execute_and_grub_lines()
}

pub fn remove(packages: Vec<String>) -> anyhow::Result<()> {
    sudo_pacman().arg("-Rs").args(packages).execute()?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    sudo_paru_or_pacman()?
        .arg("-Syu")
        .args(packages)
        .execute()?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    initialize_temp_db()?;
    paru_or_pacman()?
        .args(["-Qu", "--dbpath", TEMP_DB_PATH])
        .execute()?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qdtq").execute_and_grub_lines()
}

pub fn remvoe_orphaned_packages() -> anyhow::Result<()> {
    let orphaned_packages = orphaned_packages()?;
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

pub fn cache_size() -> anyhow::Result<()> {
    let conf = pacmanconf()?;
    let mut total_size = 0;
    for cache_dir in conf.cache_dir {
        for entry in fs_err::read_dir(Path::new(&cache_dir))? {
            let entry = entry?;
            total_size += entry.metadata()?.size();
        }
    }
    let size = humansize::format_size(total_size, humansize::DECIMAL);
    println!("{size}");
    Ok(())
}
