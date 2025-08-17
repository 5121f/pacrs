// SPDX-License-Identifier: GPL-3.0-only

use crate::cmds::{pacman, paru_if_present, paru_or_pacman, paru_or_sudo_pacman, sudo_pacman};
use crate::temp_db::TempAlpm;
use crate::utils::{is_root, paru_cache_dir, sure};
use crate::{PacrsAlpm, clean};
use crate::{pacman, temp_db};

use anyhow::bail;
use fs_err as fs;
use owo_colors::OwoColorize;

pub fn installed_pkgs() -> anyhow::Result<()> {
    pacman::installed_packages().execute()?;
    Ok(())
}

pub fn package_search(regex: &str) -> anyhow::Result<()> {
    pacman().args(["-Qs", regex]).execute()?;
    Ok(())
}

pub fn info(package: &str, recursive_deps: bool) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    if recursive_deps {
        let deps = alpm.recursive_dependencies(package);
        for dep in deps {
            println!("{}", dep.name());
        }
        return Ok(());
    }
    if alpm.is_installed_pkg(package) {
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

pub fn clean_cache(keep: u8, show_remove_candidates: bool) -> anyhow::Result<()> {
    if show_remove_candidates {
        let remove_candidates = clean::remove_candidates(keep)?;
        if remove_candidates.is_empty() {
            println!("No candidates to remove");
            return Ok(());
        }
        clean::show_cache(&remove_candidates, false)?;
        return Ok(());
    }
    if keep == 0 {
        sudo_pacman().arg("-Scc").execute()?;
        return Ok(());
    }
    clean::clean(keep)?;
    Ok(())
}

pub fn clean_cache_uninstalled() -> anyhow::Result<()> {
    sudo_pacman().arg("-Sc").execute()?;
    Ok(())
}

pub fn install(packages: Vec<String>) -> anyhow::Result<()> {
    let alpm = PacrsAlpm::new()?;
    let alpm_tmp = TempAlpm::with_default_path()?;

    let outdated_pkgs = alpm.outdated_pkgs(&alpm_tmp);

    let mut recursive_pkgs = packages.clone();
    for package in &packages {
        let deps = alpm
            .recursive_dependencies(package)
            .into_iter()
            .map(|d| d.name().to_owned())
            .collect();
        recursive_pkgs = [recursive_pkgs, deps].concat();
    }
    let has_outdated = outdated_pkgs
        .into_iter()
        .any(|outdated| recursive_pkgs.iter().any(|instelled| instelled == outdated));

    if has_outdated {
        bail!(
            "one or more package you will want to install or their dependencies was updated in \
            the repo. Update your system with 'pacrs update' before install it."
        );
    }

    paru_or_sudo_pacman().arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn list_aur_pkgs() -> anyhow::Result<Vec<String>> {
    Ok(pacman().arg("-Qmq").execute_and_grub_lines()?)
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
    let temp_db_path = temp_db::path()?;
    temp_db::init(&temp_db_path)?;
    paru_or_pacman()
        .args(["-Qu", "--dbpath", &temp_db_path.to_string_lossy()])
        .execute()?;
    Ok(())
}

pub fn orphaned_pkgs() -> anyhow::Result<Vec<String>> {
    let pkgs = pacman().arg("-Qdtq").execute_and_grub_lines()?;
    Ok(pkgs)
}

pub fn autoremove() -> anyhow::Result<()> {
    if let Some(paru) = paru_if_present() {
        paru.arg("-c").execute()?;
        return Ok(());
    }
    let orphaned_packages = orphaned_pkgs()?;
    remove(&orphaned_packages, true)
}

pub fn explicit_pkgs() -> anyhow::Result<Vec<String>> {
    let pkgs = pacman().arg("-Qeq").execute_and_grub_lines()?;
    Ok(pkgs)
}

pub fn files_of_installed_pkgs() -> anyhow::Result<()> {
    pacman().arg("-Ql").execute()?;
    Ok(())
}

pub fn deps() -> anyhow::Result<Vec<String>> {
    let deps = pacman().arg("-Qdq").execute_and_grub_lines()?;
    Ok(deps)
}

pub fn update_files_index(quiet: bool) -> anyhow::Result<()> {
    if is_root() {
        pacman().arg("-Fy").execute()?;
    } else if !quiet {
        eprintln!(
            "{}: Running without root privileges. Files index wouldn't be updated.",
            "Warning".yellow()
        );
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
