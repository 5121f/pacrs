use core::str;
use std::{path::Path, process::Command};

use crate::cmd::{execute, execute_without_output, ignure_error};

use alpm::Alpm;
use alpm_utils::DbListExt;
use anyhow::{anyhow, bail};
use fs_err as fs;

const TEMP_DB_PATH: &str = "/tmp/pacrs/db";

pub fn list() -> anyhow::Result<()> {
    let mut cmd = Command::new("pacman");
    cmd.arg("-Qq");
    execute(&mut cmd)?;
    Ok(())
}

pub fn info(package: String) -> anyhow::Result<()> {
    const COMMAND: &str = "pacman";
    let mut pacman = Command::new(COMMAND);
    pacman.args(["-Qi", &package]);
    let exit_status = ignure_error(&mut pacman)?;
    let exit_code = exit_status
        .code()
        .ok_or_else(|| anyhow!("Failed to execute {COMMAND}"))?;
    if exit_code == 0 {
        return Ok(());
    }
    let mut pacman = Command::new(COMMAND);
    pacman.args(["-Si", &package]);
    execute(&mut pacman)?;
    Ok(())
}

fn alpm_with_db_path(db_path: &str) -> Alpm {
    let conf = pacmanconf::Config::new().unwrap();
    let mut alpm = Alpm::new(&*conf.root_dir, db_path).unwrap();
    alpm_utils::configure_alpm(&mut alpm, &conf).unwrap();
    alpm
}

fn alpm() -> Alpm {
    let conf = pacmanconf::Config::new().unwrap();
    alpm_utils::alpm_with_conf(&conf).unwrap()
}

fn package_was_updated_in_db(package: &str) -> bool {
    let alpm = alpm();
    let alpm_tmp = alpm_with_db_path(TEMP_DB_PATH);
    let pkg = alpm.syncdbs().pkg(package).unwrap();
    let pkg_tmp = alpm_tmp.syncdbs().pkg(package).unwrap();
    pkg.version() < pkg_tmp.version()
}

pub fn install(packages: Vec<String>) -> anyhow::Result<()> {
    update_temp_db()?;
    for pkg in &packages {
        if package_was_updated_in_db(pkg) {
            bail!("One or more package you will want to install was updated in the repo. Upgrade your system befor install it.");
        }
    }
    let mut cmd = Command::new("paru");
    cmd.arg("-S").args(packages);
    execute(&mut cmd)?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    let mut pacman = Command::new("paru");
    pacman.arg("-Syu").args(packages);
    execute(&mut pacman)?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    update_temp_db()?;
    let mut cmd = Command::new("pacman");
    cmd.args(["-Qu", "--dbpath", TEMP_DB_PATH]);
    execute(&mut cmd)?;
    Ok(())
}

fn update_temp_db() -> anyhow::Result<()> {
    fs::create_dir_all(TEMP_DB_PATH)?;
    let conf = pacmanconf::Config::new().unwrap();
    let temp_local_db = Path::new(TEMP_DB_PATH).join("local");
    if !temp_local_db.exists() {
        fs_err::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db)?;
    }

    let mut cmd = Command::new("fakeroot");
    cmd.args(["--", "pacman", "-Sy", "--dbpath", TEMP_DB_PATH]);
    execute_without_output(&mut cmd)?;
    Ok(())
}
