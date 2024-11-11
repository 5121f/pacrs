mod alpm;
mod args;
// mod aur;
mod cmds;
mod command;
mod pacrs;
mod temp_db;

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
        Args::Upgrade { packages } => pacrs::upgrade(packages)?,
        Args::Info { package } => pacrs::info(package)?,
        Args::Search { package } => pacrs::search(package)?,
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
    let mut changed = false;
    let mut list = Vec::new();

    if updated {
        list = list_filter(list, pacrs::check_for_updates()?, changed);
        changed = true;
    }
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
