// SPDX-License-Identifier: GPL-3.0-only

use std::fmt::Display;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread::JoinHandle;

use anyhow::{Context, anyhow};
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
    Ok(etcetera::choose_base_strategy()
        .context("failed to find paru cache dir")?
        .cache_dir()
        .join("paru"))
}

pub fn confirm_from_user(message: impl Display) -> anyhow::Result<bool> {
    print!("{message} [y/N] ");
    io::stdout().flush().context("failed to flush stdout")?;
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .context("failed to read user input")?;
    let answer = buf.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}

pub trait JoinError<T> {
    fn join_err_map(self) -> anyhow::Result<T>;
}

impl<T> JoinError<T> for JoinHandle<T> {
    fn join_err_map(self) -> anyhow::Result<T> {
        self.join()
            .map_err(|err| anyhow!("thread panicked: {err:?}"))
    }
}
