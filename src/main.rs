mod alpm;
mod args;
mod cmds;
mod command;
mod pacrs;
mod temp_db;
mod utils;

use crate::{alpm::PacrsAlpm, args::Args, command::Cmd};

use anyhow::bail;
use args::{CacheCleanGroup, MarkGroup, RemoveGroup};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::Packages {
            updated,
            orphaned,
            aur,
        } => list(updated, orphaned, aur)?,
        Args::Install { packages } => pacrs::install(packages)?,
        Args::Remove(RemoveGroup { packages, orphaned }) => remove(packages, orphaned)?,
        Args::Upgrade { packages, quiet } => pacrs::upgrade(packages, quiet)?,
        Args::Info { package } => pacrs::info(package)?,
        Args::Search { package } => pacrs::search(package)?,
        Args::Files {
            package,
            find: file,
            not_update_index,
            quiet,
        } => files(package, file, not_update_index, quiet)?,
        Args::Cache { clean, size } => cache(clean, size)?,
        Args::Mark {
            packages,
            mark_group:
                MarkGroup {
                    explicit,
                    dependencie,
                },
        } => mark(packages, explicit, dependencie)?,
    }
    Ok(())
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
    pacrs::list_of_all_files()
}

fn cache(clean: CacheCleanGroup, size: bool) -> anyhow::Result<()> {
    if size {
        return pacrs::cache_size();
    }
    if clean.clean {
        if clean.uninstalled {
            return pacrs::cache_clean_uninstalled();
        }
        return pacrs::cache_clean();
    }
    Ok(())
}

fn remove(packages: Vec<String>, orphaned: bool) -> anyhow::Result<()> {
    if orphaned {
        return pacrs::remvoe_orphaned_packages();
    }
    pacrs::remove(packages)
}

fn list_filter(list: Vec<String>, packages: Vec<String>, changed: bool) -> Vec<String> {
    if !(changed || packages.is_empty()) {
        return packages;
    }
    list.into_iter()
        .filter(|line| packages.contains(line))
        .collect()
}

fn list(updated: bool, orphaned: bool, aur: bool) -> anyhow::Result<()> {
    if updated {
        pacrs::check_for_updates()?;
    }

    let mut changed = false;
    let mut list = Vec::new();

    if orphaned {
        list = list_filter(list, pacrs::orphaned_packages()?, changed);
        changed = true;
    }
    if aur {
        list = list_filter(list, pacrs::list_aur()?, changed);
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
