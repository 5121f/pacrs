// SPDX-License-Identifier: GPL-3.0-only

use std::{
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Context;
use map_self::MapSelf;
use nix::unistd::getuid;

use crate::command::Cmd;

pub fn which(program: &str) -> Option<Cmd> {
    which::which(program).is_ok().then(|| Cmd::new(program))
}

pub fn is_root() -> bool {
    getuid().is_root()
}

pub fn paru_cache_dir() -> anyhow::Result<PathBuf> {
    dirs::cache_dir()
        .context("Failed to find paru cache dir")?
        .join("paru")
        .map_self(Ok)
}

pub fn sure(message: &str) -> anyhow::Result<bool> {
    print!("{message} [y/N] ");
    io::stdout().flush().context("Failed to read input")?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .context("Failed to read user input")?;
    let answer = buf.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}
