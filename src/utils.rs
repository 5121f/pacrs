// SPDX-License-Identifier: GPL-3.0-only

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

pub trait JoinError<T> {
    fn join_err_map(self) -> anyhow::Result<T>;
}

impl<T> JoinError<T> for JoinHandle<T> {
    fn join_err_map(self) -> anyhow::Result<T> {
        self.join()
            .map_err(|err| anyhow!("thread panicked: {err:?}"))
    }
}
