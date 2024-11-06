mod args;
mod pacman;

use crate::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    match args {
        Args::List => pacman::list(),
        Args::Install { packages } => pacman::install(packages),
    }
}
