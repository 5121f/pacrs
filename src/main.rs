mod alpm;
mod args;
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
            upgradable,
            orphaned,
            aur,
        } => list(upgradable, orphaned, aur)?,
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

fn list(updated: bool, orphaned: bool, aur: bool) -> anyhow::Result<()> {
    if updated {
        return pacrs::check_for_updates();
    }
    if orphaned {
        return pacrs::orphaned_packages();
    }
    if aur {
        return pacrs::list_aur();
    }
    pacrs::list()
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
