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
        Args::Remove { packages } => pacman::remove(packages),
        Args::Upgrade { packages } => pacman::upgrade(packages),
        Args::Info { package } => pacman::info(package),
        Args::Search { package } => pacman::search(package),
    }
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
