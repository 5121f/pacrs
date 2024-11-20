mod alpm;
mod args;
mod cmds;
mod command;
mod pacman;
mod pacrs;
mod ps;
mod temp_db;
mod utils;

use alpm::PacrsAlpm;
use args::Args;
use command::Cmd;
use ps::ps;

use anyhow::bail;
use args::{MarkGroup, RemoveTarget};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::Packages {
            orphaned,
            aur,
            explicit,
            deps,
        } => list(orphaned, aur, explicit, deps)?,
        Args::Install { packages } => pacrs::install(packages)?,
        Args::Remove {
            remove_target,
            clean_deps,
        } => remove(remove_target, clean_deps)?,
        Args::Update { packages, quiet } => update(packages, quiet),
        Args::Info { package } => pacrs::info(package)?,
        Args::Search { package } => pacrs::search(package)?,
        Args::ListUpdates => pacrs::list_updates()?,
        Args::Files {
            package,
            find: file,
            not_update_index,
            quiet,
        } => files(package, file, not_update_index, quiet)?,
        Args::Clean { uninstalled, aur } => cache(uninstalled, aur)?,
        Args::Mark {
            packages,
            mark_group:
                MarkGroup {
                    explicit,
                    dependencie,
                },
        } => mark(packages, explicit, dependencie)?,
        Args::Ps => ps().await?,
    }
    Ok(())
}

fn update(packages: Vec<String>, quiet: bool) {
    let result = pacrs::update(packages);

    match result {
        Ok(()) if !quiet => eprintln!(
            "Reminder: if update system was aborted or error ends, \
            you need to finish the update before installing packages"
        ),
        Err(error) => {
            eprintln!("{error}");
            eprintln!(
                "Warning: The update ended with an error. \
                You need to finish update before installing packages."
            );
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}

fn files(
    package: Option<String>,
    file: Option<String>,
    not_update_index: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    let update_index = !not_update_index;
    if update_index {
        pacrs::update_files_index(quiet)?;
    }

    if let Some(package) = package {
        return pacrs::list_files_of_package(&package);
    }
    if let Some(file) = file {
        return pacrs::find_file(&file);
    }

    pacrs::files_of_installed_pkgs()?;
    Ok(())
}

fn cache(uninstalled: bool, aur: bool) -> anyhow::Result<()> {
    if uninstalled {
        return pacrs::cache_clean_uninstalled();
    }
    if aur {
        return pacrs::clean_paru_cache();
    }
    pacrs::cache_clean()?;
    println!("You can also clean AUR cache with 'pacrs clean --aur'");
    Ok(())
}

fn remove(remove_target: RemoveTarget, clean_deps: bool) -> anyhow::Result<()> {
    if remove_target.unneeded {
        return pacrs::remvoe_unneeded_packages(clean_deps);
    }
    pacrs::remove(remove_target.packages, clean_deps)
}

fn list_filter(list: &mut Vec<String>, packages: Vec<String>, changed: bool) {
    if !(changed || packages.is_empty()) {
        *list = packages;
        return;
    }
    list.retain(|line| packages.contains(line))
}

fn list(orphaned: bool, aur: bool, explicit: bool, deps: bool) -> anyhow::Result<()> {
    let mut changed = false;
    let mut list = Vec::new();

    if orphaned {
        list_filter(&mut list, pacrs::orphaned_packages()?, changed);
        changed = true;
    }
    if aur {
        list_filter(&mut list, pacrs::list_aur()?, changed);
        changed = true;
    }
    if explicit {
        list_filter(&mut list, pacrs::list_explicit_packages()?, changed);
        changed = true;
    }
    if deps {
        list_filter(&mut list, pacrs::list_deps()?, changed);
        changed = true;
    }

    if !changed {
        pacrs::list()?;
        return Ok(());
    }

    for package in list {
        println!("{package}");
    }

    Ok(())
}

fn mark(packages: Vec<String>, explicit: bool, dependencie: bool) -> anyhow::Result<()> {
    if explicit {
        return pacrs::mark_explicit(packages);
    }
    if dependencie {
        return pacrs::mark_dep(packages);
    }
    bail!("No one parameter specified");
}
