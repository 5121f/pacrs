use core::str;
use std::path::Path;

use crate::Cmd;

use alpm::{Alpm, Package};
use alpm_utils::DbListExt;
use anyhow::{anyhow, bail, Context};
use fs_err as fs;

const TEMP_DB_PATH: &str = "/tmp/pacrs/db";
const PACMAN_BIN: &str = "pacman";
const PARU_BIN: &str = "paru";

pub fn list() -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Qq").execute()?;
    Ok(())
}

pub fn info(package: String) -> anyhow::Result<()> {
    const COMMAND: &str = PACMAN_BIN;
    let exit_status = Cmd::new(COMMAND).args(["-Qi", &package]).ignore_error()?;
    let exit_code = exit_status
        .code()
        .ok_or_else(|| anyhow!("Failed to execute {COMMAND}"))?;
    if exit_code == 0 {
        return Ok(());
    }
    Cmd::new(COMMAND).args(["-Si", &package]).execute()?;
    Ok(())
}

pub fn search(package: String) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).args(["-Ss", &package]).execute()?;
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
    Cmd::new(PARU_BIN).arg("-S").args(packages).execute()?;
    Ok(())
}

pub fn remove(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Rs").args(packages).execute()?;
    Ok(())
}

pub fn upgrade(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PARU_BIN).arg("-Syu").args(packages).execute()?;
    Ok(())
}

pub fn check_for_updates() -> anyhow::Result<()> {
    update_temp_db()?;
    Cmd::new(PACMAN_BIN)
        .args(["-Qu", "--dbpath", TEMP_DB_PATH])
        .execute()?;
    Ok(())
}

pub fn orphaned_packages() -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN).arg("-Qdtq").execute()?;
    Ok(())
}

pub fn remvoe_orphaned_packages() -> anyhow::Result<()> {
    let orphaned_packages = Cmd::new(PACMAN_BIN)
        .arg("-Qdtq")
        .execute_and_grub_output()?
        .split("\n")
        .map(|line| line.to_owned())
        .collect();
    remove(orphaned_packages)?;
    Ok(())
}

pub fn mark_explicit(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN)
        .args(["-D", "--asexplicit"])
        .args(packages)
        .execute()?;
    Ok(())
}

pub fn mark_dep(packages: Vec<String>) -> anyhow::Result<()> {
    Cmd::new(PACMAN_BIN)
        .args(["-S", "--asdeps"])
        .args(packages)
        .execute()?;
    Ok(())
}

fn update_temp_db() -> anyhow::Result<()> {
    fs::create_dir_all(TEMP_DB_PATH)?;
    let conf = pacmanconf::Config::new().unwrap();
    let temp_local_db = Path::new(TEMP_DB_PATH).join("local");
    if !temp_local_db.exists() {
        fs_err::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db)?;
    }

    Cmd::new("fakeroot")
        .args(["--", PACMAN_BIN, "-Sy", "--dbpath", TEMP_DB_PATH])
        .execute_without_output()?;
    Ok(())
}
