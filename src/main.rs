// SPDX-License-Identifier: GPL-3.0-only

#![warn(clippy::pedantic)]

mod alpm;
mod args;
mod clean;
mod cmds;
mod command;
mod files;
mod pacman;
mod pacrs;
mod ps;
mod temp_db;
mod utils;

use alpm::PacrsAlpm;
use args::Args;
use command::Cmd;
use files::{find_file, package_files};
use pacrs::package_search;
use ps::ps;

use anyhow::bail;
use args::MarkGroup;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    match args {
        Args::Packages {
            search,
            orphaned,
            aur,
            explicit,
            deps,
        } => packages(search, orphaned, aur, explicit, deps)?,
        Args::Install { packages } => pacrs::install(&packages)?,
        Args::Remove {
            packages,
            clean_deps,
        } => pacrs::remove(&packages, clean_deps)?,
        Args::Autoremove => pacrs::autoremove()?,
        Args::Update { packages, quiet } => update(&packages, quiet),
        Args::Info { package } => pacrs::info(&package)?,
        Args::Search { package } => pacrs::search(&package)?,
        Args::ListUpdates => pacrs::list_updates()?,
        Args::Files {
            package,
            find: file,
            not_update_index,
            quiet,
        } => files(package, file, not_update_index, quiet)?,
        Args::Clean {
            uninstalled,
            aur,
            keep,
            show_remove_candidates,
        } => cache(uninstalled, aur, keep, show_remove_candidates)?,
        Args::Mark {
            packages,
            mark_group:
                MarkGroup {
                    explicit,
                    dependencie,
                },
        } => mark(&packages, explicit, dependencie)?,
        Args::Ps {
            sort_by,
            shorter,
            reverse,
            quiet,
        } => ps(sort_by.as_deref(), shorter, reverse, quiet)?,
    }
    Ok(())
}

fn update(packages: &[String], quiet: bool) {
    match pacrs::update(packages) {
        Ok(()) if !quiet => eprintln!(
            "Reminder: if update system was aborted or error ends, \
            you need to finish the update before installing packages"
        ),
        Ok(()) => {}
        Err(error) => {
            eprintln!("{error}");
            eprintln!(
                "Warning: The update ended with an error. \
                You need to finish update before installing packages."
            );
            std::process::exit(1);
        }
    }
}

fn files(
    package: Option<String>,
    file: Option<String>,
    not_update_index: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    let update_index = !not_update_index;

    if let Some(package) = package {
        return package_files(&package, update_index, quiet);
    }

    if let Some(file) = file {
        return find_file(&file, update_index, quiet);
    }

    pacrs::files_of_installed_pkgs()?;
    Ok(())
}

fn cache(
    uninstalled: bool,
    aur: bool,
    keep: u8,
    show_remove_candidates: bool,
) -> anyhow::Result<()> {
    if uninstalled {
        return pacrs::clean_cache_uninstalled();
    }
    if aur {
        return pacrs::clean_paru_cache();
    }
    pacrs::clean_cache(keep, show_remove_candidates)?;
    if !show_remove_candidates {
        println!("You can also clean AUR cache with 'pacrs clean --aur'");
    }
    Ok(())
}

fn list_filter(list: &mut Vec<String>, packages: Vec<String>, changed: bool) {
    if !changed && !packages.is_empty() {
        *list = packages;
        return;
    }
    list.retain(|line| packages.contains(line));
}

#[allow(clippy::fn_params_excessive_bools)]
fn packages(
    search_regex: Option<String>,
    orphaned: bool,
    aur: bool,
    explicit: bool,
    deps: bool,
) -> anyhow::Result<()> {
    match search_regex {
        Some(search_regex) => package_search(&search_regex),
        None => package_list(orphaned, aur, explicit, deps),
    }
}

#[allow(clippy::fn_params_excessive_bools)]
fn package_list(orphaned: bool, aur: bool, explicit: bool, deps: bool) -> anyhow::Result<()> {
    let mut changed = false;
    let mut list = Vec::new();

    if orphaned {
        list_filter(&mut list, pacrs::orphaned_pkgs()?, changed);
        changed = true;
    }
    if aur {
        list_filter(&mut list, pacrs::list_aur_pkgs()?, changed);
        changed = true;
    }
    if explicit {
        list_filter(&mut list, pacrs::explicit_pkgs()?, changed);
        changed = true;
    }
    if deps {
        list_filter(&mut list, pacrs::deps()?, changed);
        changed = true;
    }

    if !changed {
        pacrs::installed_pkgs()?;
        return Ok(());
    }

    for package in list {
        println!("{package}");
    }

    Ok(())
}

fn mark(packages: &[String], explicit: bool, dependencie: bool) -> anyhow::Result<()> {
    if explicit {
        return pacrs::mark_as_explicit(packages);
    }
    if dependencie {
        return pacrs::mark_as_dep(packages);
    }
    bail!("No one parameter specified");
}
