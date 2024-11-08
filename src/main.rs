mod args;
mod cmd;
mod pacman;

use crate::args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::List => pacman::list()?,
        Args::Install { packages } => pacman::install(packages)?,
        Args::Upgrade { packages } => pacman::upgrade(packages)?,
        Args::Info { package } => pacman::info(package)?,
        Args::Search { package } => pacman::search(package)?,
        Args::CheckForUpdates => pacman::check_for_updates()?,
    }
    Ok(())
}
