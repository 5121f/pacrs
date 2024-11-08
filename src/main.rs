mod args;
mod cmd;
mod pacman;

use crate::args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::List { upgradable } => list(upgradable)?,
        Args::Install { packages } => pacman::install(packages)?,
        Args::Remove { packages } => pacman::remove(packages)?,
        Args::Upgrade { packages } => pacman::upgrade(packages)?,
        Args::Info { package } => pacman::info(package)?,
        Args::Search { package } => pacman::search(package)?,
    }
    Ok(())
}

fn list(updated: bool) -> anyhow::Result<()> {
    if updated {
        pacman::check_for_updates()?;
        return Ok(());
    }
    pacman::list()
}
