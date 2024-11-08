mod args;
mod cmd;
mod pacman;

use crate::args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::List {
            upgradable,
            orphaned,
        } => list(upgradable, orphaned),
        Args::Install { packages } => pacman::install(packages),
        Args::Remove { packages, orphaned } => remove(packages, orphaned),
        Args::Upgrade { packages } => pacman::upgrade(packages),
        Args::Info { package } => pacman::info(package),
        Args::Search { package } => pacman::search(package),
    }
}

fn remove(packages: Vec<String>, orphaned: bool) -> anyhow::Result<()> {
    if orphaned {
        return pacman::remvoe_orphaned_packages();
    }
    pacman::remove(packages)
}

fn list(updated: bool, orphaned: bool) -> anyhow::Result<()> {
    if updated {
        return pacman::check_for_updates();
    }
    if orphaned {
        return pacman::orphaned_packages();
    }
    pacman::list()
}
