// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;

use crate::{
    Cmd,
    alpm::{PacrsAlpm, pacmanconf},
    cmds::PACMAN_BIN,
};

use alpm::Alpm;
use anyhow::Context;
use derive_more::Deref;
use fs_err as fs;

pub const TEMP_DB_PATH: &str = "/tmp/pacrs/db";

#[derive(Deref)]
pub struct TempAlpm(PacrsAlpm);

impl TempAlpm {
    pub fn new() -> anyhow::Result<Self> {
        let conf = pacmanconf()?;
        let mut alpm = Alpm::new(&*conf.root_dir, TEMP_DB_PATH)
            .context("Failed to initialize alpm connection")?;
        alpm_utils::configure_alpm(&mut alpm, &conf).context("Failed to configure alpm")?;
        initialize_temp_db()?;
        Ok(Self(PacrsAlpm::with_alpm(alpm)))
    }
}

pub fn initialize_temp_db() -> anyhow::Result<()> {
    fs::create_dir_all(TEMP_DB_PATH)?;
    let conf = pacmanconf()?;
    let temp_local_db = Path::new(TEMP_DB_PATH).join("local");
    if !temp_local_db.exists() {
        fs::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db)?;
    }
    update_temp_db()
}

pub fn update_temp_db() -> anyhow::Result<()> {
    Cmd::new("fakeroot")
        .args(["--", PACMAN_BIN, "-Sy", "--dbpath", TEMP_DB_PATH])
        .hide_output()
        .execute()?;
    Ok(())
}
