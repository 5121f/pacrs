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
        Args::CheckForUpdates => pacman::check_for_updates()?,
        Args::Upgrade => pacman::upgrade()?,
    }
    Ok(())
}
