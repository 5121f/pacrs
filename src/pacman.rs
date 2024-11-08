use core::str;
use std::{path::Path, process::Command};

use crate::cmd::{execute, execute_and_grub_output, execute_without_output, ignure_error};

use alpm::{Alpm, Package};
use alpm_utils::DbListExt;
use anyhow::{anyhow, bail, Context};
use fs_err as fs;

const TEMP_DB_PATH: &str = "/tmp/pacrs/db";
const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";

pub fn list() -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.arg("-Qq");
    execute(&mut cmd)?;
    Ok(())
}

pub fn info(package: String) -> anyhow::Result<()> {
    const COMMAND: &str = PACMAN_BIN;
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

pub fn search(package: String) -> anyhow::Result<()> {
    let mut pacman = Command::new(PACMAN_BIN);
    pacman.args(["-Ss", &package]);
    execute(&mut pacman)?;
    Ok(())
}

fn pacmanconf() -> anyhow::Result<pacmanconf::Config> {
    pacmanconf::Config::new().context("Failed to read pacmanconf")
}

fn alpm_with_db_path(db_path: &str) -> anyhow::Result<Alpm> {
    let conf = pacmanconf()?;
    let mut alpm =
        Alpm::new(&*conf.root_dir, db_path).context("Failed to initialize alpm connection")?;
    alpm_utils::configure_alpm(&mut alpm, &conf).context("Failed to configure alpm")?;
    Ok(alpm)
}

fn alpm() -> anyhow::Result<Alpm> {
    let conf = pacmanconf()?;
    alpm_utils::alpm_with_conf(&conf).context("Failed to initialize alpm connection")
}

fn package_was_updated_in_db(package: &str) -> anyhow::Result<bool> {
    let alpm = alpm()?;
    let alpm_tmp = alpm_with_db_path(TEMP_DB_PATH)?;
    let pkg = syncdb_pkg(&alpm, package)?;
    let pkg_tmp = syncdb_pkg(&alpm_tmp, package)?;
    Ok(pkg.version() < pkg_tmp.version())
}

fn syncdb_pkg<'a>(alpm: &'a Alpm, package: &str) -> anyhow::Result<&'a Package> {
    alpm.syncdbs()
        .pkg(package)
        .context("Package {package} not found")
}

pub fn install(packages: Vec<String>) -> anyhow::Result<()> {
    update_temp_db()?;
    for pkg in &packages {
        if package_was_updated_in_db(pkg)? {
            bail!("One or more package you will want to install was updated in the repo. Upgrade your system befor install it.");
        }
    }
    let mut cmd = Command::new(PARU_BIN);
    cmd.arg("-S").args(packages);
    execute(&mut cmd)?;
    Ok(())
}

pub fn remove(packages: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.arg("-Rs").args(packages);
    execute(&mut cmd)?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = Command::new(PARU_BIN);
    cmd.arg("-Syu").args(packages);
    execute(&mut cmd)?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    update_temp_db()?;
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.args(["-Qu", "--dbpath", TEMP_DB_PATH]);
    execute(&mut cmd)?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.arg("-Qdtq");
    execute(&mut cmd)?;
    Ok(())
}

pub fn remvoe_orphaned_packages() -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.arg("-Qdtq");
    let orphaned_packages = execute_and_grub_output(&mut cmd)?
        .split("\n")
        .map(|line| line.to_owned())
        .collect();
    remove(orphaned_packages)?;
    Ok(())
}

pub fn mark_explicit(packages: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.args(["-D", "--asexplicit"]).args(packages);
    execute(&mut cmd)?;
    Ok(())
}

pub fn mark_dep(packages: Vec<String>) -> anyhow::Result<()> {
    let mut cmd = Command::new(PACMAN_BIN);
    cmd.args(["-S", "--asdeps"]).args(packages);
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
    cmd.args(["--", PACMAN_BIN, "-Sy", "--dbpath", TEMP_DB_PATH]);
    execute_without_output(&mut cmd)?;
    Ok(())
}
