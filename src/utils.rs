// SPDX-License-Identifier: GPL-3.0-only

use std::fmt::Display;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread::JoinHandle;

use anyhow::{Context, anyhow};
use apply::Apply;
use etcetera::BaseStrategy;
use nix::unistd::getuid;

use crate::command::Cmd;

pub fn which(program: &str) -> Option<Cmd> {
    which::which(program).is_ok().then(|| Cmd::new(program))
}

pub fn is_root() -> bool {
    getuid().is_root()
}

pub fn paru_cache_dir() -> anyhow::Result<PathBuf> {
    etcetera::choose_base_strategy()
        .context("Failed to find paru cache dir")?
        .cache_dir()
        .join("paru")
        .apply(Ok)
}

fn sure_(message: impl Display) -> Result<bool, io::Error> {
    print!("{message} [y/N] ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let answer = buf.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}

pub fn sure(message: impl Display) -> anyhow::Result<bool> {
    sure_(message).context("Failed to read user input")
}

pub trait JoinError<T> {
    fn join_err_map(self) -> anyhow::Result<T>;
}

impl<T> JoinError<T> for JoinHandle<T> {
    fn join_err_map(self) -> anyhow::Result<T> {
        self.join()
            .map_err(|err| anyhow!("Thread paniced: {err:?}"))
    }
}
