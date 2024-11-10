mod alpm;
mod args;
mod command;
mod pacman;
mod pacrs;
mod temp_db;

use crate::{
    alpm::{PacrsAlpm, TempAlpm},
    args::Args,
    command::Cmd,
};

use anyhow::bail;
use args::{CacheCleanGroup, MarkGroup, RemoveGroup};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::List {
            upgradable,
            orphaned,
        } => list(upgradable, orphaned)?,
        Args::Install { packages } => pacrs::install(packages)?,
        Args::Remove(RemoveGroup { packages, orphaned }) => remove(packages, orphaned)?,
        Args::Upgrade { packages } => pacrs::upgrade(packages)?,
        Args::Info { package } => pacrs::info(package)?,
        Args::Search { package } => pacrs::search(package)?,
        Args::Cache { clean } => cache(clean)?,
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

fn cache(clean: CacheCleanGroup) -> anyhow::Result<()> {
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

fn list(updated: bool, orphaned: bool) -> anyhow::Result<()> {
    if updated {
        return pacrs::check_for_updates();
    }
    if orphaned {
        return pacrs::orphaned_packages();
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
