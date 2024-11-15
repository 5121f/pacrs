use std::{
    error::Error,
    fmt::Display,
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

pub fn paru_cache_dir() -> Result<PathBuf, ParuCacheDirNotFound> {
    Ok(dirs::cache_dir().ok_or(ParuCacheDirNotFound)?.join("paru"))
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

#[derive(Debug)]
pub struct ParuCacheDirNotFound;

impl Display for ParuCacheDirNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to find paru cache dir")
    }
}

impl Error for ParuCacheDirNotFound {}
