// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    PacrsAlpm,
    cmds::{pacman, paru_if_present, paru_or_pacman, paru_or_sudo_pacman, sudo_pacman},
    pacman,
    temp_db::{TEMP_DB_PATH, TempAlpm, initialize_temp_db},
    utils::{is_root, paru_cache_dir, sure},
};

use anyhow::{Context, bail};
use fs_err as fs;
use map_self::MapSelf;

pub fn installed_pkgs() -> anyhow::Result<()> {
    pacman::installed_packages().execute()?;
    Ok(())
}

pub fn info(package: &str) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    let is_local_pkg = alpm.localdb().pkg(package).is_ok();
    if is_local_pkg {
        pacman().args(["-Qi", package]).execute()?;
    } else {
        paru_or_pacman().args(["-Si", package]).execute()?;
    }
    Ok(())
}

pub fn search(package: &str) -> anyhow::Result<()> {
    paru_or_pacman().args(["-Ss", package]).execute()?;
    Ok(())
}

pub fn clean_cache() -> anyhow::Result<()> {
    pacman().arg("-Scc").execute()?;
    Ok(())
}

pub fn clean_cache_uninstalled() -> anyhow::Result<()> {
    pacman().arg("-Sc").execute()?;
    Ok(())
}

pub fn install(packages: &[String]) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    let alpm_tmp = TempAlpm::new()?;

    let pkgs = packages.iter().map(String::as_str).collect();

    let installed_pkgs = pacman::installed_packages().execute_and_grub_lines()?;
    let updated_pkgs = alpm.pkgs_or_their_deps_was_updated_in_db(&alpm_tmp, pkgs);

    let all_updated_pkgs_is_installed = updated_pkgs
        .iter()
        .all(|updated| installed_pkgs.iter().any(|instelled| instelled == updated));

    if !all_updated_pkgs_is_installed {
        bail!(
            "One or more package you will want to install or their dependencies was updated in \
            the repo. Update your system with 'pacrs update' before install it."
        );
    }

    paru_or_sudo_pacman().arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn list_aur_pkgs() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qmq").execute_and_grub_lines()?.map_self(Ok)
}

pub fn remove(packages: &[String], clean_deps: bool) -> anyhow::Result<()> {
    let mut pacman = sudo_pacman().arg("-R");
    if clean_deps {
        pacman = pacman.arg("-s");
    }
    pacman.args(packages).execute()?;
    Ok(())
}

pub fn update(packages: &[String]) -> anyhow::Result<()> {
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

pub fn orphaned_pkgs() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qdtq").execute_and_grub_lines()?.map_self(Ok)
}

pub fn autoremove() -> anyhow::Result<()> {
    if let Some(paru) = paru_if_present() {
        paru.arg("-c").execute()?;
        return Ok(());
    }
    let orphaned_packages = orphaned_pkgs()?;
    remove(&orphaned_packages, true)?;
    Ok(())
}

pub fn explicit_pkgs() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qeq").execute_and_grub_lines()?.map_self(Ok)
}

pub fn files_of_installed_pkgs() -> anyhow::Result<()> {
    pacman::files_of_installed_pkgs().execute()?;
    Ok(())
}

pub fn deps() -> anyhow::Result<Vec<String>> {
    pacman().arg("-Qdq").execute_and_grub_lines()?.map_self(Ok)
}

pub fn parse_pacman_files_output(lines: &[String]) -> anyhow::Result<Vec<String>> {
    let mut res = Vec::with_capacity(lines.len());

    for line in lines {
        let pkg_name = line
            .split_ascii_whitespace()
            .nth(1)
            .context("Failed to parse pacman output")?;
        res.push(pkg_name.to_owned());
    }

    Ok(res)
}

pub fn update_files_index(quiet: bool) -> anyhow::Result<()> {
    if is_root() {
        pacman().arg("-Fy").execute()?;
        return Ok(());
    }
    if !quiet {
        eprintln!("Running without root privileges. Files index wouldn't be updated.");
    }
    Ok(())
}

pub fn mark_as_explicit(packages: &[String]) -> anyhow::Result<()> {
    pacman()
        .args(["-D", "--asexplicit"])
        .args(packages)
        .execute()?;
    Ok(())
}

pub fn mark_as_dep(packages: &[String]) -> anyhow::Result<()> {
    pacman().args(["-D", "--asdeps"]).args(packages).execute()?;
    Ok(())
}

pub fn clean_paru_cache() -> anyhow::Result<()> {
    if !sure("You really wont to delete AUR (paru) cache?")? {
        return Ok(());
    }
    let cache_dir = paru_cache_dir()?;
    fs::remove_dir_all(cache_dir)?;
    Ok(())
}
