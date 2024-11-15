use std::{
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Context;
use nix::unistd::getuid;
use which::which;

pub fn program_is_present(program: &str) -> bool {
    which(program).is_ok()
}

pub fn is_root() -> bool {
    getuid().is_root()
}

pub fn paru_cache_dir() -> anyhow::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .context("Failed to find paru cache dir")?
        .join("paru"))
}

pub fn shure(message: &str) -> anyhow::Result<bool> {
    print!("{message} [y/N] ");
    io::stdout().flush().context("Failed to read input")?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .context("Failed to read input")?;
    Ok(buf.trim().to_lowercase() == "y")
}
