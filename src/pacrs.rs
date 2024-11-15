use crate::{
    cmds::{pacman, paru_if_present, paru_or_pacman, paru_or_sudo_pacman, sudo_pacman},
    temp_db::{initialize_temp_db, TempAlpm, TEMP_DB_PATH},
    utils::{is_root, paru_cache_dir, shure},
    PacrsAlpm,
};

use anyhow::{bail, Ok};

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
    paru_or_pacman().args(["-Si", &package]).execute()?;
    Ok(())
}

pub fn search(package: String) -> anyhow::Result<()> {
    paru_or_pacman().args(["-Ss", &package]).execute()?;
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
    let pkgs = packages.iter().map(String::as_str).collect();
    if alpm.pkgs_or_their_deps_was_updated_in_db(&alpm_tmp, pkgs)? {
        bail!(
            "One or more package you will want to install or their dependencies was updated in \
            the repo. Upgrade your system with 'pacrs upgrade' befor install it."
        );
    }
    paru_or_sudo_pacman().arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn list_aur() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qmq").execute_and_grub_lines()
}

pub fn remove(packages: Vec<String>, clean_deps: bool) -> anyhow::Result<()> {
    let mut pacman = sudo_pacman().arg("-R");
    if clean_deps {
        pacman = pacman.arg("-s");
    }
    pacman.args(packages).execute()?;
    Ok(())
}

pub fn update(packages: Vec<String>) -> anyhow::Result<()> {
    paru_or_sudo_pacman().arg("-Syu").args(packages).execute()?;
    Ok(())
}

pub fn list_updates() -> anyhow::Result<()> {
    initialize_temp_db()?;
    paru_or_pacman()
        .args(["-Qu", "--dbpath", TEMP_DB_PATH])
        .execute()?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qdtq").execute_and_grub_lines()
}

pub fn remvoe_orphaned_packages(clean_deps: bool) -> anyhow::Result<()> {
    if let Some(paru) = paru_if_present() {
        paru.arg("-c").execute()?;
        return Ok(());
    }
    let orphaned_packages = orphaned_packages()?;
    remove(orphaned_packages, clean_deps)?;
    Ok(())
}

pub fn find_file(file: &str) -> anyhow::Result<()> {
    pacman().arg("-F").arg(file).execute()?;
    Ok(())
}

pub fn list_of_all_files() -> anyhow::Result<()> {
    pacman().arg("-Fl").execute()?;
    Ok(())
}

pub fn list_explicit_packages() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qeq").execute_and_grub_lines()
}

pub fn list_deps() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qdq").execute_and_grub_lines()
}

pub fn list_files_of_package(name: &str) -> anyhow::Result<()> {
    pacman().arg("-Fl").arg(name).execute()?;
    Ok(())
}

pub fn update_files_index(quiet: bool) -> anyhow::Result<()> {
    if is_root() {
        pacman().arg("-Fy").execute()?;
        return Ok(());
    }
    if !quiet {
        eprintln!(
            "Info: you run program without root priviliges. \
            Files index wouldn't be updated."
        );
    }
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

pub fn clean_paru_cache() -> anyhow::Result<()> {
    if !shure("You really wont to delete AUR (paru) cache?")? {
        return Ok(());
    }
    let cache_dir = paru_cache_dir()?;
    fs_err::remove_dir_all(cache_dir)?;
    Ok(())
}
