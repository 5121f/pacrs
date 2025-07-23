// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use alpm::Alpm;
use anyhow::Context;
use apply::Apply;
use derive_more::Deref;
use etcetera::BaseStrategy;
use fs_err as fs;

use crate::Cmd;
use crate::alpm::{PacrsAlpm, pacmanconf};
use crate::cmds::PACMAN_BIN;

pub fn path() -> anyhow::Result<PathBuf> {
    Ok(etcetera::base_strategy::choose_base_strategy()?
        .cache_dir()
        .join("pacrs/db"))
}

#[derive(Deref)]
pub struct TempAlpm(PacrsAlpm);

impl TempAlpm {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let conf = pacmanconf()?;
        let mut alpm = Alpm::new(&*conf.root_dir, &path.to_string_lossy())
            .context("Failed to initialize alpm connection")?;
        alpm_utils::configure_alpm(&mut alpm, &conf).context("Failed to configure alpm")?;
        init(path)?;
        Self(PacrsAlpm::with_alpm(alpm)).apply(Ok)
    }

    pub fn with_default_path() -> anyhow::Result<Self> {
        let path = path()?;
        let tmp_alpm = Self::new(path)?;
        Ok(tmp_alpm)
    }
}

pub fn init(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let temp_db_path = path.as_ref();
    fs::create_dir_all(temp_db_path)?;
    let conf = pacmanconf()?;
    let temp_local_db = temp_db_path.join("local");
    if !temp_local_db.exists() {
        let local_db = Path::new(&conf.db_path).join("local");
        fs::os::unix::fs::symlink(local_db, temp_local_db)?;
    }
    update(temp_db_path.to_string_lossy())
}

pub fn update(path: impl AsRef<str>) -> anyhow::Result<()> {
    Cmd::new("fakeroot")
        .args(["--", PACMAN_BIN, "-Sy", "--dbpath", path.as_ref()])
        .hide_output()
        .execute()?;
    Ok(())
}
