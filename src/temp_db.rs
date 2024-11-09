use std::path::Path;

use crate::{pacman::PACMAN_BIN, Cmd};

use fs_err as fs;

pub const TEMP_DB_PATH: &str = "/tmp/pacrs/db";

pub fn initialize_temp_db() -> anyhow::Result<()> {
    fs::create_dir_all(TEMP_DB_PATH)?;
    let conf = pacmanconf::Config::new().unwrap();
    let temp_local_db = Path::new(TEMP_DB_PATH).join("local");
    if !temp_local_db.exists() {
        fs_err::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db)?;
    }
    update_temp_db()
}

pub fn update_temp_db() -> anyhow::Result<()> {
    Cmd::new("fakeroot")
        .args(["--", PACMAN_BIN, "-Sy", "--dbpath", TEMP_DB_PATH])
        .execute_without_output()?;
    Ok(())
}
